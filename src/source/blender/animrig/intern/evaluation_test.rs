// Converted from [blender/source/blender/animrig/intern/evaluation_test] via Qwen2.5-Coder

use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Copy)]
struct Main {
    // Define fields as needed
}

#[derive(Debug, Clone, Copy)]
struct Action {
    slot_add: fn() -> Slot,
    layer_add: fn(&str) -> Layer,
}

#[derive(Debug, Clone, Copy)]
struct Object {
    id_new: fn(&Main, &str) -> Self,
    add_only_object: fn(&Main, i32, &str) -> Self,
    loc: [f32; 3],
    rot: [f32; 3],
}

#[derive(Debug, Clone, Copy)]
struct Slot {
    handle: usize,
}

#[derive(Debug, Clone, Copy)]
struct Layer {
    strip_add: fn(&Action, Strip::Type) -> Strip,
    resize: fn(&mut Strip, f32, f32),
}

#[derive(Debug, Clone, Copy)]
enum StripType {
    Keyframe,
}

#[derive(Debug, Clone, Copy)]
struct Strip {
    data: fn(&Strip, &Action) -> StripKeyframeData,
}

#[derive(Debug, Clone, Copy)]
struct StripKeyframeData {
    keyframe_insert: fn(&Main, &Slot, &str, &[f32], KeyframeSettings),
}

#[derive(Debug, Clone, Copy)]
struct KeyframeSettings {
    interpolation: i32,
}

#[derive(Debug, Clone, Copy)]
struct EvaluationResult {
    result_: HashMap<PropIdentifier, AnimatedProperty>,
}

impl EvaluationResult {
    fn store(&mut self, rna_path: &str, array_index: usize, value: f32, _fake_resolved_rna: PathResolvedRNA) {
        let key = PropIdentifier::new(rna_path, array_index);
        self.result_.insert(key, AnimatedProperty { value });
    }

    fn lookup_ptr(&self, key: PropIdentifier) -> Option<&AnimatedProperty> {
        self.result_.get(&key)
    }
}

#[derive(Debug, Clone, Copy)]
struct AnimatedProperty {
    value: f32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct PropIdentifier {
    rna_path: CString,
    array_index: usize,
}

impl PropIdentifier {
    fn new(rna_path: &str, array_index: usize) -> Self {
        PropIdentifier {
            rna_path: CString::new(rna_path).unwrap(),
            array_index,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PathResolvedRNA;

mod tests {
    use super::*;

    #[test]
    fn prop_identifier_hashing() {
        let mut result = EvaluationResult { result_: HashMap::new() };

        let rna_path_1 = "pose.bones['Root'].location";
        let rna_path_2 = CString::new(rna_path_1).unwrap();
        assert_ne!(rna_path_1, rna_path_2.as_bytes());

        let fake_resolved_rna = PathResolvedRNA;
        result.store(rna_path_1, 0, 1.0f, fake_resolved_rna);
        result.store(&rna_path_2, 0, 2.0f, fake_resolved_rna);
        assert_eq!(1, result.result_.len());

        {
            let key = PropIdentifier::new(rna_path_1, 0);
            if let Some(anim_prop) = result.lookup_ptr(key) {
                assert_eq!(2.0f, anim_prop.value);
            }
        }

        {
            let key = PropIdentifier::new(rna_path_2.as_bytes(), 0);
            if let Some(anim_prop) = result.lookup_ptr(key) {
                assert_eq!(2.0f, anim_prop.value);
            }
        }
    }
}
