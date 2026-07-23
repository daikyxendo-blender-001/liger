// Converted from [blender/source/blender/animrig/intern/action_legacy_test] via Qwen2.5-Coder

use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Copy)]
struct Main {
    // Define the fields of Main here
}

impl Main {
    fn new() -> Self {
        // Implement the logic to create a new Main instance
        Main {}
    }

    fn free(&self) {
        // Implement the logic to free the Main instance
    }
}

#[derive(Debug, Clone, Copy)]
struct bAction {
    curves: Vec<FCurve>,
    // Define other fields of bAction here
}

impl bAction {
    fn new(bmain: &Main, name: &str) -> Self {
        // Implement the logic to create a new bAction instance
        bAction { curves: Vec::new() }
    }

    fn add_curve(&mut self, fcurve: FCurve) {
        self.curves.push(fcurve);
    }
}

#[derive(Debug, Clone, Copy)]
struct FCurve {
    array_index: i32,
    // Define other fields of FCurve here
}

impl FCurve {
    fn new() -> Self {
        // Implement the logic to create a new FCurve instance
        FCurve { array_index: 0 }
    }

    fn set_rnapath(&mut self, rna_path: &str) {
        // Implement the logic to set the RNA path for the FCurve
    }
}

#[derive(Debug, Clone, Copy)]
struct Slot {
    handle: u32,
    // Define other fields of Slot here
}

impl Slot {
    fn unassigned() -> Self {
        // Implement the logic to create an unassigned Slot instance
        Slot { handle: 0 }
    }

    fn add(&self) -> Slot {
        // Implement the logic to add a new Slot
        Slot { handle: self.handle + 1 }
    }
}

#[derive(Debug, Clone, Copy)]
struct StripKeyframeData {
    channelbag_for_slot_ensure: fn(slot: &Slot) -> Channelbag,
    fcurve_ensure: fn(bmain: Option<&Main>, path: &[u8], array_index: i32) -> FCurve,
}

impl StripKeyframeData {
    fn new() -> Self {
        // Implement the logic to create a new StripKeyframeData instance
        StripKeyframeData {
            channelbag_for_slot_ensure: |slot| Channelbag::new(slot),
            fcurve_ensure: |bmain, path, array_index| FCurve::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Channelbag {
    fcurves: Vec<FCurve>,
}

impl Channelbag {
    fn new(_slot: &Slot) -> Self {
        // Implement the logic to create a new Channelbag instance
        Channelbag { fcurves: Vec::new() }
    }

    fn add_fcurve(&mut self, fcurve: FCurve) {
        self.fcurves.push(fcurve);
    }

    fn fcurves(&self) -> &[FCurve] {
        &self.fcurves
    }
}

#[derive(Debug, Clone, Copy)]
struct Action {
    slots: Vec<Slot>,
    layer_keystrip_ensure: fn(&mut Self),
    strip_keyframe_data: fn(&Self) -> Vec<&StripKeyframeData>,
}

impl Action {
    fn new() -> Self {
        // Implement the logic to create a new Action instance
        Action {
            slots: Vec::new(),
            layer_keystrip_ensure: |self_| self_.layer_keystrip_ensure_impl(),
            strip_keyframe_data: |self_| self_.strip_keyframe_data_impl(),
        }
    }

    fn add_slot(&mut self) -> Slot {
        let slot = Slot::unassigned();
        self.slots.push(slot);
        slot
    }

    fn layer_keystrip_ensure_impl(&mut self) {
        // Implement the logic to ensure a layer and keystrip
    }

    fn strip_keyframe_data_impl(&self) -> Vec<&StripKeyframeData> {
        vec![&StripKeyframeData::new()]
    }
}

fn fcurves_all(action: Option<&Action>) -> Vec<&FCurve> {
    if let Some(action) = action {
        action.strip_keyframe_data().iter().flat_map(|strip| strip.fcurves.iter()).collect()
    } else {
        Vec::new()
    }
}

fn action_fcurves_remove(action: &mut Action, slot_handle: u32, path: &str) -> bool {
    if let Some(strip_data) = action.strip_keyframe_data().iter_mut().find(|strip| strip.channelbag_for_slot_ensure(&Slot { handle: slot_handle }).fcurve_ensure(None, path.as_bytes(), 0).is_some()) {
        strip_data.channelbag_for_slot_ensure(&Slot { handle: slot_handle }).fcurve_ensure(None, path.as_bytes(), 0) = None;
        true
    } else {
        false
    }
}

fn main() {
    let mut bmain = Main::new();
    let action = Action::new();

    // Add slots and curves to the action
    let slot1 = action.add_slot();
    let slot2 = action.add_slot();

    action.layer_keystrip_ensure_impl();

    let fcurve1 = action.strip_keyframe_data().iter_mut().find(|strip| strip.channelbag_for_slot_ensure(&slot1).fcurve_ensure(None, b"location\0", 0).is_none()).unwrap().channelbag_for_slot_ensure(&slot1).fcurve_ensure(None, b"location\0", 0);
    let fcurve2 = action.strip_keyframe_data().iter_mut().find(|strip| strip.channelbag_for_slot_ensure(&slot2).fcurve_ensure(None, b"scale\0", 0).is_none()).unwrap().channelbag_for_slot_ensure(&slot2).fcurve_ensure(None, b"scale\0", 0);

    let fcurves_expect = vec![&fcurve1, &fcurve2];
    assert_eq!(fcurves_expect, fcurves_all(Some(&action)));

    assert!(!action_fcurves_remove(&mut action, slot1.handle, "rotation"));
}
