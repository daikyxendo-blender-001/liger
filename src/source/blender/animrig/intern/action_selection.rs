// Converted from [blender/source/blender/animrig/intern/action_selection] via Qwen2.5-Coder

use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Action {
    fcurves: Vec<FCurve>,
}

#[derive(Debug, Clone, Copy)]
struct FCurve {
    keys: Vec<Key>,
}

#[derive(Debug, Clone, Copy)]
struct Key {
    selected: bool,
}

impl Action {
    fn deselect_keys(&self) {
        for fcu in self.fcurves.iter() {
            fcu.deselect_all_keys();
        }
    }
}

impl FCurve {
    fn deselect_all_keys(&mut self) {
        for key in self.keys.iter_mut() {
            key.selected = false;
        }
    }
}

fn deselect_keys_actions(actions: &[&Action]) {
    let mut visited_actions = HashSet::new();
    for action in actions {
        if !visited_actions.insert(action) {
            continue;
        }
        action.deselect_keys();
    }
}
