#!/usr/bin/env python3
"""
Blender C/C++ -> Rust Bulk Converter (Pairing .h + .cpp -> .rs without Cargo Check)
"""

import os
import sys
import json
import argparse
import urllib.request
import urllib.error
import re
import subprocess
import tempfile
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime

MANIFEST_PATH = os.path.join(os.path.dirname(__file__), "converted_manifest.json")
WORKSPACE_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
BLENDER_DIR = os.path.join(WORKSPACE_ROOT, "blender")
SRC_DIR = os.path.join(WORKSPACE_ROOT, "src")

SHORT_SYSTEM_PROMPT = (
    "Role: Expert C/C++ to Rust Transpiler.\n"
    "Task: Combine C/C++ header (.h) and source (.cpp/.c) into ONE clean Rust file (.rs).\n\n"
    "Rules:\n"
    "1. Structure: Combine declarations and logic into `struct` + `impl` blocks. Add `#[derive(Debug, Clone, Copy)]` where applicable.\n"
    "2. Naming: Types/Structs -> PascalCase, Functions/Vars -> snake_case, Constants -> SCREAMING_SNAKE_CASE.\n"
    "3. Types: C pointers/arrays -> Slices `&[T]` / `Vec<T>`, NULL/errors -> `Option<T>` / `Result<T, E>`.\n"
    "4. Output: Return ONLY the valid Rust code block inside ```rust ... ```. No explanations.\n"
)

def load_manifest():
    if os.path.exists(MANIFEST_PATH):
        with open(MANIFEST_PATH, "r", encoding="utf-8") as f:
            return json.load(f)
    return {"processed_files": [], "failed_files": [], "total_converted": 0, "last_run": None}

def save_manifest(manifest):
    manifest["last_run"] = datetime.now().isoformat()
    with open(MANIFEST_PATH, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, ensure_ascii=False)

def call_ollama(prompt, model="deepseek-coder-v2:16b", ollama_url="http://localhost:11434", timeout=300):
    url = f"{ollama_url.rstrip('/')}/api/generate"
    payload = {
        "model": model,
        "prompt": prompt,
        "stream": False,
        "options": {
            "temperature": 0.2
        }
    }
    data = json.dumps(payload).encode("utf-8")
    try:
        req = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
        with urllib.request.urlopen(req, timeout=timeout) as response:
            resp_body = response.read().decode("utf-8")
            res_json = json.loads(resp_body)
            return res_json.get("response", "")
    except Exception as e:
        print(f"Error calling Ollama API ({url}): {e}")
        return None

def extract_rust_code_block(llm_output):
    """Extract code inside ```rust ... ``` block if present."""
    match = re.search(r"```(?:rust|rs)\s*(.*?)\s*```", llm_output, re.DOTALL | re.IGNORECASE)
    if match:
        return match.group(1).strip()
    return None

def validate_rust_code(rust_code):
    if not re.search(r"\b(fn|struct|enum|impl|trait|use|mod|const|type|pub)\b", rust_code):
        return False, "output does not look like Rust code"

    tmp_path = None
    try:
        with tempfile.NamedTemporaryFile("w", encoding="utf-8", suffix=".rs", delete=False) as f:
            tmp_path = f.name
            f.write(rust_code)

        result = subprocess.run(
            ["rustfmt", "--edition", "2024", "--check", tmp_path],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
        )
        if result.returncode == 0:
            return True, None

        detail = " ".join(result.stdout.strip().splitlines()[:3])
        return False, f"rustfmt --check failed: {detail}"
    except FileNotFoundError:
        return False, "rustfmt is not installed"
    finally:
        if tmp_path and os.path.exists(tmp_path):
            os.unlink(tmp_path)

def find_candidate_file_groups(manifest, max_count=20):
    processed = set(manifest.get("processed_files", []))
    source_root = os.path.join(BLENDER_DIR, "source")
    if not os.path.exists(source_root):
        print(f"Warning: Blender source directory not found at {source_root}")
        return []

    # Map (directory_rel_path, base_name) -> dict of extensions {'h': full_path, 'cpp': full_path}
    groups = {}

    for root, _, files in os.walk(source_root):
        rel_dir = os.path.relpath(root, WORKSPACE_ROOT)
        for file in sorted(files):
            ext = os.path.splitext(file)[1].lower()
            if ext in ('.h', '.hpp', '.c', '.cpp', '.cc'):
                base = os.path.splitext(file)[0]
                key = (rel_dir, base)
                if key not in groups:
                    groups[key] = {'header': None, 'source': None, 'key_str': f"{rel_dir}/{base}"}
                
                full_path = os.path.join(root, file)
                if ext in ('.h', '.hpp'):
                    groups[key]['header'] = full_path
                else:
                    groups[key]['source'] = full_path

    # Sort groups deterministically by path
    sorted_keys = sorted(groups.keys(), key=lambda k: groups[k]['key_str'])
    
    candidates = []
    for key in sorted_keys:
        group_info = groups[key]
        key_str = group_info['key_str']
        if key_str not in processed:
            candidates.append(group_info)
            if len(candidates) >= max_count:
                break

    return candidates

def convert_file_group(group_info, args):
    key_str = group_info['key_str']
    header_path = group_info['header']
    source_path = group_info['source']

    print(f"\n==========================================")
    print(f"Converting [{key_str}]...")

    combined_code = ""
    if header_path and os.path.exists(header_path):
        with open(header_path, "r", encoding="utf-8", errors="ignore") as f:
            combined_code += f"// HEADER FILE: {os.path.basename(header_path)}\n" + f.read() + "\n\n"

    if source_path and os.path.exists(source_path):
        with open(source_path, "r", encoding="utf-8", errors="ignore") as f:
            combined_code += f"// SOURCE FILE: {os.path.basename(source_path)}\n" + f.read() + "\n"

    if not combined_code.strip():
        print(f"Skipping empty group {key_str}")
        return False

    base_prompt = f"{SHORT_SYSTEM_PROMPT}\n\nC/C++ Source Code for [{key_str}]:\n```cpp\n{combined_code}\n```"
    attempts = args.retries + 1
    rust_code = None
    last_error = None

    for attempt in range(1, attempts + 1):
        prompt = base_prompt
        if last_error:
            prompt += (
                "\n\nPrevious response was rejected because: "
                f"{last_error}\n"
                "Return exactly one ```rust fenced code block with syntactically valid Rust. "
                "Do not return prose, JSON, Markdown lists, or C/C++."
            )

        print(f"Calling Ollama API for {key_str} ({attempt}/{attempts}, timeout={args.request_timeout}s)...")
        llm_response = call_ollama(
            prompt,
            model=args.model,
            ollama_url=args.ollama_url,
            timeout=args.request_timeout,
        )
        if not llm_response:
            last_error = "empty Ollama response"
        else:
            rust_code = extract_rust_code_block(llm_response)
            if rust_code is None:
                last_error = "missing ```rust fenced code block"
            else:
                is_valid, validation_error = validate_rust_code(rust_code)
                if is_valid:
                    break
                last_error = validation_error
                rust_code = None

        print(f"Rejected output for {key_str}: {last_error}")
        if attempt < attempts:
            time.sleep(min(30, 5 * attempt))

    if rust_code is None:
        print(f"Failed to produce valid Rust for {key_str}: {last_error}")
        return False

    # Determine target output path in src/
    rel_dir = group_info['key_str'].rsplit('/', 1)[0]
    base_name = group_info['key_str'].rsplit('/', 1)[1]
    
    # Remove 'blender/' prefix if present in rel_dir
    clean_dir = rel_dir
    if clean_dir.startswith("blender/"):
        clean_dir = clean_dir[len("blender/"):]

    target_dir = os.path.join(SRC_DIR, clean_dir)
    os.makedirs(target_dir, exist_ok=True)
    target_rs_path = os.path.join(target_dir, f"{base_name}.rs")

    # Save generated Rust file directly
    with open(target_rs_path, "w", encoding="utf-8") as f:
        f.write(f"// Converted from [{key_str}] via {args.model}\n\n{rust_code}\n")

    print(f"SAVED -> [{os.path.relpath(target_rs_path, WORKSPACE_ROOT)}]")
    return True

def main():
    parser = argparse.ArgumentParser(description="Bulk convert Blender C/C++ pairs (.h + .cpp) to Rust.")
    parser.add_argument("--batch-size", type=int, default=20, help="Number of file pairs to convert.")
    parser.add_argument("--model", type=str, default="deepseek-coder-v2:16b", help="Ollama model name.")
    parser.add_argument("--ollama-url", type=str, default="http://localhost:11434", help="Ollama API base URL.")
    parser.add_argument("--workers", type=int, default=1, help="Number of parallel conversion workers.")
    parser.add_argument("--request-timeout", type=int, default=300, help="Ollama request timeout in seconds.")
    parser.add_argument("--retries", type=int, default=0, help="Retry attempts per file after an Ollama failure.")
    args = parser.parse_args()
    args.workers = max(1, args.workers)
    args.request_timeout = max(1, args.request_timeout)
    args.retries = max(0, args.retries)

    manifest = load_manifest()
    candidates = find_candidate_file_groups(manifest, max_count=args.batch_size)

    if not candidates:
        print("No new C/C++ file groups found to convert!")
        return

    print(
        f"Starting bulk conversion of {len(candidates)} file groups using model "
        f"[{args.model}] with {args.workers} worker(s)..."
    )

    converted_count = 0

    def record_result(key_str, success):
        nonlocal converted_count
        if success:
            if key_str not in manifest["processed_files"]:
                manifest["processed_files"].append(key_str)
            if key_str in manifest["failed_files"]:
                manifest["failed_files"].remove(key_str)
            manifest["total_converted"] = len(manifest["processed_files"])
            converted_count += 1
        else:
            if key_str not in manifest["failed_files"]:
                manifest["failed_files"].append(key_str)

        save_manifest(manifest)

    if args.workers == 1:
        for group_info in candidates:
            key_str = group_info['key_str']
            success = convert_file_group(group_info, args)
            record_result(key_str, success)
    else:
        with ThreadPoolExecutor(max_workers=args.workers) as executor:
            future_to_group = {
                executor.submit(convert_file_group, group_info, args): group_info
                for group_info in candidates
            }
            for future in as_completed(future_to_group):
                group_info = future_to_group[future]
                key_str = group_info['key_str']
                try:
                    success = future.result()
                except Exception as e:
                    print(f"Error converting {key_str}: {e}")
                    success = False
                record_result(key_str, success)

    print(f"\nBulk conversion finished. Successfully converted {converted_count}/{len(candidates)} groups.")

if __name__ == "__main__":
    main()
