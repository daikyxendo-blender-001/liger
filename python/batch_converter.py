#!/usr/bin/env python3
"""
Blender C/C++ -> Rust Batch Converter using Qwen2.5-Coder & Cargo Verification.
"""

import os
import sys
import json
import argparse
import subprocess
import re
import urllib.request
import urllib.error
from datetime import datetime

MANIFEST_PATH = os.path.join(os.path.dirname(__file__), "converted_manifest.json")
WORKSPACE_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
BLENDER_DIR = os.path.join(WORKSPACE_ROOT, "blender")
SRC_DIR = os.path.join(WORKSPACE_ROOT, "src")

def load_manifest():
    if os.path.exists(MANIFEST_PATH):
        with open(MANIFEST_PATH, "r", encoding="utf-8") as f:
            return json.load(f)
    return {"processed_files": [], "failed_files": [], "total_converted": 0, "last_run": None}

def save_manifest(manifest):
    manifest["last_run"] = datetime.now().isoformat()
    with open(MANIFEST_PATH, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, ensure_ascii=False)

def safe_ensure_module_import(parent_filepath, child_mod_name):
    """
    Ensures that parent_filepath contains `pub mod <child_mod_name>;`.
    If file doesn't exist, creates it.
    If file exists, appends the line if not already present. NEVER overwrites existing code.
    """
    os.makedirs(os.path.dirname(parent_filepath), exist_ok=True)
    mod_declaration = f"pub mod {child_mod_name};"
    
    if not os.path.exists(parent_filepath):
        with open(parent_filepath, "w", encoding="utf-8") as f:
            f.write(f"// Auto-generated module tree\n{mod_declaration}\n")
        print(f"Created module file: {parent_filepath} -> added {mod_declaration}")
        return

    with open(parent_filepath, "r", encoding="utf-8") as f:
        content = f.read()

    # Check if module is already declared (pub mod child_name; or mod child_name;)
    pattern = rf"\b(pub\s+)?mod\s+{re.escape(child_mod_name)}\s*;"
    if not re.search(pattern, content):
        with open(parent_filepath, "a", encoding="utf-8") as f:
            if not content.endswith("\n"):
                f.write("\n")
            f.write(f"{mod_declaration}\n")
        print(f"Updated module file: {parent_filepath} -> appended {mod_declaration}")

def setup_rust_module_hierarchy(source_rel_path):
    """
    For source_rel_path like 'blender/source/blender/blenlib/BLI_math_base.h':
    Maps to 'src/source/blender/blenlib/BLI_math_base.rs'
    And ensures safe parent module declarations:
    - src/lib.rs -> pub mod source;
    - src/source/mod.rs -> pub mod blender;
    - src/source/blender/mod.rs -> pub mod blenlib;
    - src/source/blender/blenlib/mod.rs -> pub mod BLI_math_base;
    """
    # Remove 'blender/' prefix if present
    clean_path = source_rel_path
    if clean_path.startswith("blender/"):
        clean_path = clean_path[len("blender/"):]

    path_parts = clean_path.split("/")
    filename = path_parts[-1]
    mod_name = os.path.splitext(filename)[0]
    
    # Standardize module name (replace non-alphanumeric except _ with _)
    mod_name = re.sub(r'[^a-zA-Z0-9_]', '_', mod_name)
    dirs = path_parts[:-1] # e.g. ['source', 'blender', 'blenlib']

    # 1. Ensure src/lib.rs imports first level
    if dirs:
        lib_rs = os.path.join(SRC_DIR, "lib.rs")
        safe_ensure_module_import(lib_rs, dirs[0])

    # 2. Chain intermediate mod.rs files
    current_dir = SRC_DIR
    for i in range(len(dirs)):
        current_dir = os.path.join(current_dir, dirs[i])
        parent_mod_rs = os.path.join(current_dir, "mod.rs")
        if i + 1 < len(dirs):
            child_name = dirs[i + 1]
            safe_ensure_module_import(parent_mod_rs, child_name)
        else:
            # Last directory's mod.rs imports the target rust file
            safe_ensure_module_import(parent_mod_rs, mod_name)

    target_rs_path = os.path.join(current_dir, f"{mod_name}.rs")
    return target_rs_path, mod_name

def call_ollama(prompt, model="qwen2.5-coder:7b", ollama_url="http://localhost:11434"):
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
    req = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=300) as response:
            resp_body = response.read().decode("utf-8")
            res_json = json.loads(resp_body)
            return res_json.get("response", "")
    except Exception as e:
        print(f"Error calling Ollama API ({url}): {e}")
        return None

def extract_code_block(llm_output):
    """Extract code inside ```rust ... ``` block if present."""
    match = re.search(r"```rust\s*(.*?)\s*```", llm_output, re.DOTALL)
    if match:
        return match.group(1).strip()
    match = re.search(r"```\s*(.*?)\s*```", llm_output, re.DOTALL)
    if match:
        return match.group(1).strip()
    return llm_output.strip()

def run_cargo_check():
    """Runs `cargo check --lib` and returns (success_boolean, stderr_output)"""
    try:
        res = subprocess.run(
            ["cargo", "check", "--lib"],
            cwd=WORKSPACE_ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        return res.returncode == 0, res.stderr
    except Exception as e:
        return False, str(e)

def convert_single_file(source_file_path, rel_path, args):
    print(f"\n==========================================")
    print(f"Processing [{rel_path}]...")
    
    with open(source_file_path, "r", encoding="utf-8", errors="ignore") as f:
        code_content = f.read()

    target_rs_path, mod_name = setup_rust_module_hierarchy(rel_path)

    system_prompt = (
        "You are an expert C/C++ to Rust transpiler specializing in graphics and engine code (Blender).\n"
        "Convert the following C/C++ header or source file to idiomatic, safe Rust code.\n"
        "Rules:\n"
        "1. Prefer safe Rust (`Slice`, `Option`, `Result`, `enum`, `struct`). Use `unsafe` only if strictly required.\n"
        "2. Include proper `#[derive(Debug, Clone, Copy)]` where applicable.\n"
        "3. Preserve function names, constants, and math logic as closely as possible.\n"
        "4. Provide ONLY the valid, compilable Rust code block inside ```rust ... ``` block without conversational text.\n"
    )

    prompt = f"{system_prompt}\n\nFile Name: {os.path.basename(source_file_path)}\n\nC/C++ Code:\n```cpp\n{code_content}\n```"

    attempt = 0
    max_retries = args.max_retries
    compiler_error = ""

    while attempt <= max_retries:
        if attempt > 0:
            print(f"Retry attempt {attempt}/{max_retries} for {rel_path} due to cargo check error...")
            prompt = (
                f"{system_prompt}\n\nPrevious attempt failed with cargo check error:\n```\n{compiler_error}\n```\n"
                f"Please fix the compilation errors and provide the corrected, complete Rust code.\n"
            )

        llm_response = call_ollama(prompt, model=args.model, ollama_url=args.ollama_url)
        if not llm_response:
            print(f"Failed to get LLM response for {rel_path}")
            return False

        rust_code = extract_code_block(llm_response)
        
        # Save temporary generated code
        with open(target_rs_path, "w", encoding="utf-8") as f:
            f.write(f"// Converted from {rel_path} via Qwen2.5-Coder\n\n{rust_code}\n")

        # Run cargo check
        passed, stderr = run_cargo_check()
        if passed:
            print(f"SUCCESS: [{rel_path}] -> [{os.path.relpath(target_rs_path, WORKSPACE_ROOT)}] passed cargo check!")
            return True
        else:
            compiler_error = stderr
            print(f"Cargo check failed for {rel_path}")
            attempt += 1

    # Cleanup failed file to keep build clean
    if os.path.exists(target_rs_path):
        os.remove(target_rs_path)
    return False

def find_candidate_files(manifest, max_count=20):
    processed = set(manifest.get("processed_files", []))
    candidates = []

    source_root = os.path.join(BLENDER_DIR, "source")
    if not os.path.exists(source_root):
        print(f"Warning: Blender source directory not found at {source_root}")
        return candidates

    # Priority list of core Blender modules (from most fundamental to higher-level)
    priority_subdirs = [
        os.path.join(source_root, "blender", "blenlib"),     # 1. Core math, memory, data structures
        os.path.join(source_root, "blender", "makesdna"),    # 2. DNA structs (Blend file core types)
        os.path.join(source_root, "blender", "makesrna"),    # 3. RNA reflection
        os.path.join(source_root, "blender", "bmesh"),       # 4. BMesh geometry data structures
        os.path.join(source_root, "blender", "blenkernel"),  # 5. Core engine logic (Object, Scene, Mesh)
        os.path.join(source_root, "blender", "nodes"),       # 6. Geometry & Shader nodes
    ]

    all_target_dirs = [d for d in priority_subdirs if os.path.exists(d)]
    
    # Add remaining directories as fallback
    for root, dirs, _ in os.walk(source_root):
        if root not in all_target_dirs:
            all_target_dirs.append(root)

    for target_dir in all_target_dirs:
        for root, _, files in os.walk(target_dir):
            for file in sorted(files):
                if file.endswith((".h", ".hpp", ".c", ".cpp")):
                    full_path = os.path.join(root, file)
                    rel_path = os.path.relpath(full_path, WORKSPACE_ROOT)
                    if rel_path not in processed:
                        candidates.append((full_path, rel_path))
                        if len(candidates) >= max_count:
                            return candidates
    return candidates

def main():
    parser = argparse.ArgumentParser(description="Batch convert Blender C/C++ to Rust using Qwen2.5-Coder.")
    parser.add_argument("--batch-size", type=int, default=20, help="Number of files to convert in this run.")
    parser.add_argument("--model", type=str, default="qwen2.5-coder:7b", help="Ollama model name.")
    parser.add_argument("--ollama-url", type=str, default="http://localhost:11434", help="Ollama API base URL.")
    parser.add_argument("--max-retries", type=int, default=3, help="Max retries per file on compiler error.")
    args = parser.parse_args()

    manifest = load_manifest()
    candidates = find_candidate_files(manifest, max_count=args.batch_size)

    if not candidates:
        print("No new candidate files found to convert!")
        return

    print(f"Starting batch conversion of {len(candidates)} files using model [{args.model}]...")

    converted_in_this_run = 0
    for source_path, rel_path in candidates:
        success = convert_single_file(source_path, rel_path, args)
        if success:
            manifest["processed_files"].append(rel_path)
            manifest["total_converted"] = len(manifest["processed_files"])
            converted_in_this_run += 1
        else:
            manifest["failed_files"].append(rel_path)
        
        save_manifest(manifest)

    print(f"\nBatch conversion complete. Successfully converted {converted_in_this_run}/{len(candidates)} files.")

if __name__ == "__main__":
    main()
