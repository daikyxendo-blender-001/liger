// Converted from [blender/source/blender/animrig/intern/action_iterators_test] via Qwen2.5-Coder

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Main {
    // Define fields as needed
}

#[derive(Debug, Clone, Copy)]
struct Action {
    slots: Vec<Slot>,
    layers: Vec<Layer>,
}

impl Action {
    fn new() -> Self {
        Action {
            slots: Vec::new(),
            layers: Vec::new(),
        }
    }

    fn slot_add(&mut self) -> &mut Slot {
        let slot = Slot::new();
        self.slots.push(slot);
        self.slots.last_mut().unwrap()
    }

    fn layer_add(&mut self, name: &str) -> &mut Layer {
        let layer = Layer::new(name);
        self.layers.push(layer);
        self.layers.last_mut().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Slot {
    handle: u32,
    // Define other fields as needed
}

impl Slot {
    fn new() -> Self {
        Slot { handle: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
struct Layer {
    name: String,
    strips: Vec<Strip>,
}

impl Layer {
    fn new(name: &str) -> Self {
        Layer {
            name: name.to_string(),
            strips: Vec::new(),
        }
    }

    fn strip_add(&mut self, action: &Action, strip_type: StripType) -> &mut Strip {
        let strip = Strip::new(action, strip_type);
        self.strips.push(strip);
        self.strips.last_mut().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Strip {
    data: StripKeyframeData,
    // Define other fields as needed
}

impl Strip {
    fn new(_action: &Action, _strip_type: StripType) -> Self {
        Strip {
            data: StripKeyframeData::new(),
            // Initialize other fields as needed
        }
    }

    fn keyframe_insert(
        &mut self,
        bmain: &Main,
        slot: &Slot,
        rna_path: &str,
        values: [f32; 2],
        settings: KeyframeSettings,
    ) -> SingleKeyingResult {
        // Implement logic to insert keyframe
        SingleKeyingResult::SUCCESS
    }
}

#[derive(Debug, Clone, Copy)]
enum StripType {
    Keyframe,
    // Define other variants as needed
}

#[derive(Debug, Clone, Copy)]
struct StripKeyframeData {
    // Define fields as needed
}

impl StripKeyframeData {
    fn new() -> Self {
        StripKeyframeData {
            // Initialize fields as needed
        }
    }

    fn data(&self, _action: &Action) -> &Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
struct KeyframeSettings {
    // Define fields as needed
}

impl KeyframeSettings {
    fn new() -> Self {
        KeyframeSettings {
            // Initialize fields as needed
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SingleKeyingResult {
    SUCCESS,
    // Define other variants as needed
}

fn foreach_fcurve_in_action_slot(
    action: &Action,
    slot_handle: u32,
    mut f: impl FnMut(&FCurve),
) {
    for strip in &action.layers.iter().flat_map(|layer| layer.strips.iter()) {
        if strip.data.action == action && strip.data.slot.handle == slot_handle {
            // Implement logic to iterate FCurves
            f(&FCurve::new());
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FCurve {
    rna_path: &'static str,
    array_index: usize,
    // Define other fields as needed
}

impl FCurve {
    fn new() -> Self {
        FCurve {
            rna_path: "",
            array_index: 0,
            // Initialize other fields as needed
        }
    }
}
