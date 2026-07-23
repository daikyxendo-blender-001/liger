// Converted from [blender/source/blender/animrig/intern/action_iterators] via Qwen2.5-Coder

use std::ffi::CStr;

#[derive(Debug, Clone, Copy)]
struct Action {
    layers: Vec<Layer>,
}

impl Action {
    fn foreach_fcurve_in_action<F>(&self, callback: F)
    where
        F: FnMut(FCurve),
    {
        for layer in &self.layers {
            for strip in &layer.strips {
                if strip.strip_type != StripType::Keyframe {
                    continue;
                }
                for bag in &strip.data.channelbags {
                    for fcu in &bag.fcurves {
                        callback(fcu);
                    }
                }
            }
        }
    }

    fn foreach_fcurve_in_action_slot_editable<F>(&self, handle: slot_handle_t, callback: F)
    where
        F: FnMut(FCurve),
    {
        assert_baklava_phase_1_invariants(self);
        for layer in &self.layers {
            for strip in &layer.strips {
                if strip.strip_type != StripType::Keyframe {
                    continue;
                }
                for bag in &strip.data.channelbags {
                    if bag.slot_handle != handle {
                        continue;
                    }
                    for fcu in &bag.fcurves {
                        assert!(fcu.is_some());
                        if fcu.as_ref().unwrap().flag & FCURVE_PROTECTED != 0 {
                            continue;
                        }
                        callback(fcu.unwrap());
                    }
                }
            }
        }
    }

    fn foreach_fcurve_in_action_slot<F>(&self, handle: slot_handle_t, callback: F)
    where
        F: FnMut(FCurve),
    {
        for layer in &self.layers {
            for strip in &layer.strips {
                if strip.strip_type != StripType::Keyframe {
                    continue;
                }
                for bag in &strip.data.channelbags {
                    if bag.slot_handle != handle {
                        continue;
                    }
                    for fcu in &bag.fcurves {
                        assert!(fcu.is_some());
                        callback(fcu.unwrap());
                    }
                }
            }
        }
    }

    fn foreach_action_slot_use<F>(&self, animated_id: &ID, callback: F) -> bool
    where
        F: FnMut(&Action, slot_handle_t) -> bool,
    {
        let forward_to_callback = |animated_id: &ID, action_ptr_ref: Option<&mut Action>, slot_handle_ref: &slot_handle_t, last_slot_identifier: *mut i8| -> bool {
            if action_ptr_ref.is_none() {
                return true;
            }
            callback(action_ptr_ref.unwrap(), *slot_handle_ref)
        };

        self.foreach_action_slot_use_with_references(animated_id, forward_to_callback)
    }

    fn foreach_action_slot_use_with_references<F>(&self, animated_id: &ID, callback: F) -> bool
    where
        F: FnMut(&ID, Option<&mut Action>, &mut slot_handle_t, *mut i8) -> bool,
    {
        let adt = BKE_animdata_from_id(animated_id);
        if let Some(adt) = adt {
            if let Some(action_ptr_ref) = adt.action.as_mut() {
                if !callback(animated_id, Some(action_ptr_ref), &mut adt.slot_handle, adt.last_slot_identifier) {
                    return false;
                }
            }

            bke::nla::foreach_strip_adt(*adt, |strip| {
                if let Some(act) = strip.act.as_mut() {
                    if !callback(animated_id, Some(act), &mut strip.action_slot_handle, strip.last_slot_identifier) {
                        return false;
                    }
                }
                true
            });
        }

        if GS(animated_id.name) != ID_OB {
            return true;
        }

        let object = animated_id.as_object();
        for con in &object.constraints {
            if !self.visit_constraint(con, callback) {
                return false;
            }
        }

        if object.type_ == OB_ARMATURE {
            for pchan in &object.pose.chanbase {
                for con in &pchan.constraints {
                    if !self.visit_constraint(con, callback) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn foreach_action_slot_use_with_rna<F>(&self, animated_id: &ID, callback: F) -> bool
    where
        F: FnMut(&ID, Option<&mut Action>, PointerRNA, PropertyRNA, *mut i8) -> bool,
    {
        let adt = BKE_animdata_from_id(animated_id);
        if let Some(adt) = adt {
            if let Some(action_ptr_ref) = adt.action.as_mut() {
                let ptr = RNA_pointer_create_discrete(animated_id, RNA_AnimData, adt);
                let prop = RNA_struct_find_property(&ptr, "action_slot");
                if !callback(animated_id, Some(action_ptr_ref), ptr, *prop, adt.last_slot_identifier) {
                    return false;
                }
            }

            bke::nla::foreach_strip_adt(*adt, |strip| {
                if let Some(act) = strip.act.as_mut() {
                    let ptr = RNA_pointer_create_discrete(animated_id, RNA_NlaStrip, strip);
                    let prop = RNA_struct_find_property(&ptr, "action_slot");
                    if !callback(animated_id, Some(act), ptr, *prop, strip.last_slot_identifier) {
                        return false;
                    }
                }
                true
            });
        }

        if GS(animated_id.name) != ID_OB {
            return true;
        }

        let object = animated_id.as_object();
        for con in &object.constraints {
            if !self.visit_constraint(con, callback) {
                return false;
            }
        }

        if object.type_ == OB_ARMATURE {
            for pchan in &object.pose.chanbase {
                for con in &pchan.constraints {
                    if !self.visit_constraint(con, callback) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn visit_constraint<F>(&self, constraint: &bConstraint, callback: F) -> bool
    where
        F: FnMut(&bConstraint) -> bool,
    {
        if constraint.type_ != CONSTRAINT_TYPE_ACTION {
            return true;
        }
        let constraint_data = constraint.data.as_action_constraint();
        if constraint_data.act.is_none() {
            return true;
        }

        let ptr = RNA_pointer_create_discrete(animated_id, RNA_ActionConstraint, &constraint);
        let prop = RNA_struct_find_property(&ptr, "action_slot");
        callback(constraint)
    }
}

#[derive(Debug, Clone, Copy)]
struct Layer {
    strips: Vec<Strip>,
}

#[derive(Debug, Clone, Copy)]
enum StripType {
    Keyframe,
}

#[derive(Debug, Clone, Copy)]
struct Strip {
    strip_type: StripType,
    data: StripData,
}

#[derive(Debug, Clone, Copy)]
struct StripData {
    channelbags: Vec<Channelbag>,
}

#[derive(Debug, Clone, Copy)]
struct Channelbag {
    slot_handle: slot_handle_t,
    fcurves: Vec<FCurve>,
}

#[derive(Debug, Clone, Copy)]
enum FCurveFlag {
    Protected = 1 << 0,
}

#[derive(Debug, Clone, Copy)]
struct FCurve {
    flag: u32,
}

impl FCurve {
    fn is_some(&self) -> bool {
        self.flag & FCURVE_PROTECTED == 0
    }

    fn unwrap(&self) -> &FCurve {
        assert!(self.is_some());
        self
    }
}

#[derive(Debug, Clone, Copy)]
struct slot_handle_t;

#[derive(Debug, Clone, Copy)]
enum CONSTRAINT_TYPE_ACTION {
    Action,
}

#[derive(Debug, Clone, Copy)]
struct bConstraint {
    type_: CONSTRAINT_TYPE_ACTION,
    data: *mut std::ffi::c_void,
}

impl bConstraint {
    fn as_action_constraint(&self) -> &bActionConstraint {
        unsafe { &*(self.data as *const bActionConstraint) }
    }
}

#[derive(Debug, Clone, Copy)]
struct bActionConstraint {
    act: Option<&mut Action>,
    action_slot_handle: slot_handle_t,
    last_slot_identifier: *mut i8,
}

#[derive(Debug, Clone, Copy)]
enum IDType {
    Object = 1,
}

#[derive(Debug, Clone, Copy)]
struct ID {
    name: [u8; 64],
    type_: IDType,
}

impl ID {
    fn as_object(&self) -> &Object {
        unsafe { &*(self as *const ID as *const Object) }
    }
}

#[derive(Debug, Clone, Copy)]
struct Object {
    constraints: Vec<bConstraint>,
    pose: Option<Pose>,
    type_: IDType,
}

#[derive(Debug, Clone, Copy)]
struct Pose {
    chanbase: Vec<bPoseChannel>,
}

#[derive(Debug, Clone, Copy)]
struct bPoseChannel {
    constraints: Vec<bConstraint>,
}

extern "C" {
    fn BKE_animdata_from_id(id: *const ID) -> *mut AnimData;
    fn RNA_pointer_create_discrete(id: *const ID, type_: *const std::ffi::c_char, data: *const std::ffi::c_void) -> PointerRNA;
    fn RNA_struct_find_property(ptr: &PointerRNA, name: *const std::ffi::c_char) -> PropertyRNA;
}

#[repr(C)]
struct AnimData {
    action: Option<&mut Action>,
    slot_handle: slot_handle_t,
    last_slot_identifier: *mut i8,
}

#[repr(C)]
struct PointerRNA {
    type_: *const std::ffi::c_char,
    data: *const std::ffi::c_void,
}

#[repr(C)]
struct PropertyRNA;

fn assert_baklava_phase_1_invariants(action: &Action) {}

fn GS(name: &[u8]) -> IDType {
    if name == b"Object" {
        IDType::Object
    } else {
        IDType::Object
    }
}

fn BKE_animdata_from_id(id: *const ID) -> *mut AnimData {
    unimplemented!()
}
