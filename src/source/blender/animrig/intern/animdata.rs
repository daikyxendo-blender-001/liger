// Converted from [blender/source/blender/animrig/intern/animdata] via Qwen2.5-Coder

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct ID {
    name: String,
    flag: u32,
    us: usize,
}

impl ID {
    fn new(name: &str, flag: u32) -> Self {
        ID {
            name: name.to_string(),
            flag,
            us: 1,
        }
    }

    fn is_embedded_data(&self) -> bool {
        self.flag & ID_FLAG_EMBEDDED_DATA != 0
    }

    fn owner_get(&self) -> Option<&ID> {
        // Placeholder for actual implementation
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct Object {
    id: ID,
    data: Option<ID>,
    particlesystem: Vec<ParticleSystem>,
}

impl Object {
    fn new(id: ID, data: Option<ID>) -> Self {
        Object { id, data, particlesystem: Vec::new() }
    }

    fn add_particle_system(&mut self, particle_system: ParticleSystem) {
        self.particlesystem.push(particle_system);
    }
}

#[derive(Debug, Clone, Copy)]
struct ParticleSystem {
    part: Option<ID>,
}

impl ParticleSystem {
    fn new(part: Option<ID>) -> Self {
        ParticleSystem { part }
    }
}

#[derive(Debug, Clone, Copy)]
struct Main {
    objects: VecDeque<Object>,
}

impl Main {
    fn new() -> Self {
        Main {
            objects: VecDeque::new(),
        }
    }

    fn add_object(&mut self, object: Object) {
        self.objects.push_back(object);
    }
}

#[derive(Debug, Clone, Copy)]
struct Action {
    id: ID,
    // Placeholder for actual fields
}

impl Action {
    fn new(id: ID) -> Self {
        Action { id }
    }

    fn is_empty(&self) -> bool {
        // Placeholder for actual implementation
        false
    }

    fn fcurve_remove(&mut self, fcu: FCurve) {
        // Placeholder for actual implementation
    }
}

#[derive(Debug, Clone, Copy)]
struct AnimData {
    action: Option<Action>,
    flag: u32,
    slot_handle: usize,
}

impl AnimData {
    fn new(action: Option<Action>, flag: u32, slot_handle: usize) -> Self {
        AnimData { action, flag, slot_handle }
    }

    fn ensure_id(id: &ID) -> Option<AnimData> {
        // Placeholder for actual implementation
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct FCurve {
    driver: Option<Driver>,
    // Placeholder for actual fields
}

impl FCurve {
    fn new(driver: Option<Driver>) -> Self {
        FCurve { driver }
    }

    fn free(&self) {
        // Placeholder for actual implementation
    }
}

#[derive(Debug, Clone, Copy)]
struct Driver {
    // Placeholder for actual fields
}

fn add_object_data_users(bmain: &Main, id: &ID, related_ids: &mut Vec<&ID>) {
    if ID_REAL_USERS(id) != 1 {
        return;
    }

    for object in bmain.objects.iter() {
        if let Some(data) = object.data.as_ref() {
            if data == id {
                related_ids.push(&object.id);
            }
        }
        for particle_system in &object.particlesystem {
            if let Some(part) = particle_system.part.as_ref() {
                if part.id == *id {
                    related_ids.push(&particle_system.part.unwrap().id);
                }
            }
        }
    }
}

fn find_related_ids(bmain: &Main, id: &ID) -> Vec<&ID> {
    let mut related_ids = vec![id];
    let mut i = 0;

    while i < related_ids.len() {
        let related_id = related_ids[i];

        if related_id.is_embedded_data() {
            if let Some(owner_id) = related_id.owner_get() {
                related_ids.push(owner_id);
            }
        }

        match GS(&related_id.name) {
            ID_OB => {
                if let Some(data) = related_id.data.as_ref() {
                    if ID_REAL_USERS(data) == 1 {
                        related_ids.push(data);
                    }
                }
                for particle_system in &related_id.particlesystem {
                    if let Some(part) = particle_system.part.as_ref() {
                        if ID_REAL_USERS(&part.id) != 1 {
                            continue;
                        }
                        related_ids.push(&part.id);
                    }
                }
            }
            ID_KE => {
                // Placeholder for actual implementation
            }
            ID_MA => {
                // Placeholder for actual implementation
            }
            _ => {}
        }

        i += 1;
    }

    related_ids
}

fn assign_action(action: &Action, id_adt: (ID, AnimData)) -> bool {
    // Placeholder for actual implementation
    true
}

fn animdata_fcurve_delete(adt: Option<&AnimData>, fcu: Option<&FCurve>) {
    if let Some(adt) = adt {
        if let Some(fcu) = fcu {
            if fcu.driver.is_some() {
                // Placeholder for actual implementation
            } else if let Some(action) = adt.action.as_ref() {
                action.fcurve_remove(fcu.clone());
                return;
            }
        }
    }
}

fn animdata_remove_empty_action(adt: Option<&AnimData>) -> bool {
    if let Some(adt) = adt {
        if let Some(act) = adt.action.as_ref() {
            if act.is_empty() && (adt.flag & ADT_NLA_EDIT_ON) == 0 {
                id_us_min(&act.id);
                adt.action.take();
                return true;
            }
        }
    }

    false
}

fn fcurve_find_by_rna_path(adt: &AnimData, rna_path: &str, array_index: i32) -> Option<&FCurve> {
    if let Some(action) = adt.action.as_ref() {
        // Placeholder for actual implementation
        None
    } else {
        None
    }
}

fn fcurves_for_assigned_action(adt: Option<&AnimData>) -> Vec<&FCurve> {
    if let Some(adt) = adt {
        if let Some(action) = adt.action.as_ref() {
            // Placeholder for actual implementation
            vec![]
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

fn fcurves_for_assigned_action_const(adt: Option<&AnimData>) -> Vec<&FCurve> {
    if let Some(adt) = adt {
        if let Some(action) = adt.action.as_ref() {
            // Placeholder for actual implementation
            vec![]
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

fn id_us_min(id: &ID) {
    id.us -= 1;
}

const ID_FLAG_EMBEDDED_DATA: u32 = 0x00000001;
const ADT_NLA_EDIT_ON: u32 = 0x00000002;

fn GS(name: &str) -> char {
    name.chars().next().unwrap_or(' ')
}
