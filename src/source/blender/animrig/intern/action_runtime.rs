// Converted from [blender/source/blender/animrig/intern/action_runtime] via Qwen2.5-Coder

use std::collections::{HashMap, HashSet};
use std::ptr;

#[derive(Debug, Clone, Copy)]
struct Slot {
    runtime: Option<SlotRuntime>,
}

impl Slot {
    fn new() -> Self {
        Slot { runtime: None }
    }

    fn users_add(&mut self, id: *const ID) {
        if let Some(runtime) = &mut self.runtime {
            runtime.users.insert(id);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SlotRuntime {
    users: HashSet<*const ID>,
}

impl SlotRuntime {
    fn new() -> Self {
        SlotRuntime { users: HashSet::new() }
    }

    fn clear(&mut self) {
        self.users.clear();
    }
}

#[derive(Debug, Clone, Copy)]
struct Action {
    slots: Vec<Slot>,
}

impl Action {
    fn slot_for_handle(&self, handle: u32) -> Option<&Slot> {
        if handle < self.slots.len() as u32 {
            Some(&self.slots[handle as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Main {
    actions: Vec<bAction>,
    is_action_slot_to_id_map_dirty: bool,
}

impl Main {
    fn new() -> Self {
        Main {
            actions: Vec::new(),
            is_action_slot_to_id_map_dirty: false,
        }
    }

    fn rebuild_slot_user_cache(&mut self) {
        for dna_action in &self.actions {
            let action = Action { slots: dna_action.slots.clone() };
            for slot in &action.slots {
                if let Some(runtime) = &mut slot.runtime {
                    runtime.clear();
                }
            }
        }

        self.is_action_slot_to_id_map_dirty = false;

        let mut visited_ids = HashSet::new();

        fn visit_id(id: *const ID, visited_ids: &mut HashSet<*const ID>, action_slots: &[Slot]) -> bool {
            if !visited_ids.insert(id) {
                return false;
            }

            for slot in action_slots {
                if let Some(runtime) = &slot.runtime {
                    runtime.users.insert(id);
                }
            }

            true
        }

        for ids_of_idtype in &self.actions.iter().map(|action| &action.slots).collect::<Vec<_>>() {
            for id in ids_of_idtype.iter() {
                visit_id(id, &mut visited_ids, ids_of_idtype);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct bAction {
    slots: Vec<Slot>,
}

impl bAction {
    fn wrap(&self) -> Action {
        Action { slots: self.slots.clone() }
    }

    fn slots(&self) -> &[Slot] {
        &self.slots
    }
}

#[derive(Debug, Clone, Copy)]
struct ID;

fn id_type_can_have_animdata(id_type: u32) -> bool {
    // Placeholder implementation
    true
}

fn id_can_have_animdata(_id: *const ID) -> bool {
    // Placeholder implementation
    true
}

fn bke_node_tree_from_id(_id: *const ID) -> Option<*mut bNodeTree> {
    None
}

#[derive(Debug, Clone, Copy)]
struct bNodeTree;

fn rebuild_slot_user_cache(bmain: &mut Main) {
    bmain.rebuild_slot_user_cache();
}
