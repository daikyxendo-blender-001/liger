// Converted from [blender/source/blender/animrig/intern/anim_rna] via Qwen2.5-Coder

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr;

use fmt::format;

use blender_sys::{
    BLI_listbase,
    BLI_math_base,
    BLI_string,
    BLI_vector,
    DNA_object_types,
    RNA_access,
    RNA_path,
    RNA_prototypes,
};

#[derive(Debug, Clone, Copy)]
enum eRotationModes {
    ROT_MODE_QUAT,
    ROT_MODE_AXISANGLE,
    ROT_MODE_EUL,
}

struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector { data: Vec::new() }
    }

    fn append(&mut self, value: T) {
        self.data.push(value);
    }

    fn reinitialize(&mut self, length: usize) {
        self.data.resize(length, Default::default());
    }

    fn index_range(&self) -> std::ops::Range<usize> {
        0..self.data.len()
    }
}

struct RNAPath(String);

impl RNAPath {
    fn new(path: String) -> Self {
        RNAPath(path)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.0.starts_with(prefix)
    }

    fn ends_with(&self, suffix: &str) -> bool {
        self.0.ends_with(suffix)
    }

    fn rfind(&self, c: char) -> usize {
        self.0.rfind(c).unwrap_or(usize::MAX)
    }

    fn substr(&self, start: usize, len: usize) -> String {
        self.0[start..start + len].to_string()
    }
}

struct PointerRNA {
    data: *mut c_void,
    type_: u32,
}

impl PointerRNA {
    fn new(data: *mut c_void, type_: u32) -> Self {
        PointerRNA { data, type_ }
    }

    fn is_null(&self) -> bool {
        self.data.is_null()
    }

    fn is_not_null(&self) -> bool {
        !self.data.is_null()
    }
}

struct PropertyRNA;

impl PropertyRNA {
    fn new() -> Self {
        PropertyRNA
    }

    fn array_check(&self, ptr: &PointerRNA) -> bool {
        // Placeholder implementation
        false
    }

    fn array_length(&self, ptr: &PointerRNA) -> usize {
        // Placeholder implementation
        0
    }

    fn type_(&self) -> u32 {
        // Placeholder implementation
        0
    }
}

struct RNAPath;

impl RNAPath {
    fn resolve_property(
        ptr: &PointerRNA,
        path: &str,
        resolved_ptr: &mut PointerRNA,
        resolved_prop: &mut PropertyRNA,
    ) -> bool {
        // Placeholder implementation
        false
    }

    fn escape_find_quote(s: &str) -> Option<&str> {
        // Placeholder implementation
        None
    }
}

struct IDProperty;

impl IDProperty {
    fn new() -> Self {
        IDProperty
    }

    fn type_(&self) -> u32 {
        // Placeholder implementation
        0
    }

    fn subtype(&self) -> u32 {
        // Placeholder implementation
        0
    }
}

struct bPoseChannel;

impl bPoseChannel {
    fn new() -> Self {
        bPoseChannel
    }

    fn name(&self) -> &str {
        // Placeholder implementation
        ""
    }

    fn rotmode(&self) -> u32 {
        // Placeholder implementation
        0
    }
}

struct Object;

impl Object {
    fn new() -> Self {
        Object
    }

    fn rotmode(&self) -> u32 {
        // Placeholder implementation
        0
    }
}

fn get_rna_values(ptr: &PointerRNA, prop: &PropertyRNA) -> Vector<f32> {
    let mut values = Vector::new();
    if prop.array_check(ptr) {
        let length = prop.array_length(ptr);

        match prop.type_() {
            1 => { /* PROP_BOOLEAN */ }
            2 => { /* PROP_INT */ }
            4 => {
                // PROP_FLOAT
                values.reinitialize(length);
                unsafe {
                    RNA_property_float_get_array(ptr, prop, values.data.as_mut_ptr());
                }
            }
            _ => {}
        }
    } else {
        match prop.type_() {
            1 => { /* PROP_BOOLEAN */ }
            2 => { /* PROP_INT */ }
            4 => {
                // PROP_FLOAT
                values.append(RNA_property_float_get(ptr, prop));
            }
            5 => { /* PROP_ENUM */ }
            _ => {}
        }
    }

    values
}

const POSE_BONE_PATH_PREFIX: &str = "pose.bones[\"";
const POSE_BONE_PATH_PREFIX_LENGTH: usize = POSE_BONE_PATH_PREFIX.len();

fn get_pose_bone_rna_path(pose_bone: &bPoseChannel) -> String {
    let mut name_esc = vec![0u8; pose_bone.name.len() * 2];
    BLI_string::BLI_str_escape(&mut name_esc, pose_bone.name.as_bytes(), name_esc.len());
    format!("{}{}\"]", POSE_BONE_PATH_PREFIX, String::from_utf8_lossy(&name_esc))
}

fn pose_bone_name_from_rna_path(rna_path: &str) -> Option<String> {
    if rna_path.len() < POSE_BONE_PATH_PREFIX_LENGTH
        || !rna_path.starts_with(POSE_BONE_PATH_PREFIX)
    {
        return None;
    }

    let name_esc = &rna_path[POSE_BONE_PATH_PREFIX_LENGTH..];
    let name_esc_end = BLI_string::BLI_str_escape_find_quote(name_esc);
    if name_esc_end.is_none() {
        return None;
    }
    let name_esc_len = name_esc_end.unwrap() - name_esc.as_ptr() as usize;
    if name_esc_len >= MAXBONENAME {
        return None;
    }
    let mut name = vec![0u8; MAXBONENAME];
    BLI_string::BLI_str_unescape(&mut name, &name_esc[..name_esc_len], name.len());
    Some(String::from_utf8_lossy(&name).into_owned())
}

const MAXBONENAME: usize = 64;

fn rna_property_get_as_float(ptr: &PointerRNA, prop: &PropertyRNA) -> Vector<f32> {
    let mut values = Vector::new();
    if prop.array_check(ptr) {
        values.reinitialize(prop.array_length(ptr));
    } else {
        values.reinitialize(1);
    }
    match prop.type_() {
        1 => { /* PROP_BOOLEAN */ }
        2 => { /* PROP_INT */ }
        4 => {
            // PROP_FLOAT
            unsafe {
                RNA_property_float_get_array(ptr, prop, values.data.as_mut_ptr());
            }
        }
        _ => {}
    }
    values
}

fn rna_property_set_as_float(
    ptr: &mut PointerRNA,
    prop: &PropertyRNA,
    values: &[f32],
) {
    if prop.array_check(ptr) && prop.array_length(ptr) != values.len() {
        // Array length has to match.
        panic!("Array length has to match");
    }

    match prop.type_() {
        1 => { /* PROP_BOOLEAN */ }
        2 => { /* PROP_INT */ }
        4 => {
            // PROP_FLOAT
            unsafe {
                RNA_property_float_set_array(ptr, prop, values.as_ptr());
            }
        }
        _ => {}
    }
}
