// Converted from [blender/source/blender/animrig/intern/bonecolor] via Qwen2.5-Coder

use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::mem::size_of;

#[derive(Debug, Clone, Copy)]
pub struct ThemeWireColor {
    pub solid: [u8; 4],
    pub select: [u8; 4],
    pub active: [u8; 4],
    pub flag: u8,
}

impl PartialEq for BoneColor {
    fn eq(&self, other: &Self) -> bool {
        if self.palette_index != other.palette_index {
            return false;
        }
        if self.palette_index == -1 {
            return std::memcmp(self.custom.solid.as_ptr(), other.custom.solid.as_ptr(), size_of::<[u8; 3]>() - 1) == 0
                && std::memcmp(self.custom.select.as_ptr(), other.custom.select.as_ptr(), size_of::<[u8; 3]>() - 1) == 0
                && std::memcmp(self.custom.active.as_ptr(), other.custom.active.as_ptr(), size_of::<[u8; 3]>() - 1) == 0
                && self.custom.flag == other.custom.flag;
        }
        true
    }
}

impl Eq for BoneColor {}

impl Hash for BoneColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.palette_index >= 0 {
            get_default_hash(self.palette_index).hash(state);
        } else {
            let hash_solid = get_default_hash(self.custom.solid[0], self.custom.solid[1], self.custom.solid[2]);
            let hash_select = get_default_hash(
                self.custom.select[0], self.custom.select[1], self.custom.select[2],
            );
            let hash_active = get_default_hash(
                self.custom.active[0], self.custom.active[1], self.custom.active[2],
            );
            get_default_hash(hash_solid, hash_select, hash_active, self.custom.flag).hash(state);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoneColor {
    pub palette_index: i8,
    pub custom: ThemeWireColor,
}

impl BoneColor {
    pub fn new() -> Self {
        BoneColor {
            palette_index: 0,
            custom: ThemeWireColor {
                solid: [0; 4],
                select: [0; 4],
                active: [0; 4],
                flag: 0,
            },
        }
    }

    pub fn effective_color(&self) -> Option<&ThemeWireColor> {
        match self.palette_index {
            0 => None,
            -1 => Some(&self.custom),
            _ => {
                let btheme = ui::theme::theme_get();
                Some(&btheme.tarm[(self.palette_index as usize - 1)])
            }
        }
    }

    pub fn from_other(other: &BoneColor) -> Self {
        BoneColor {
            palette_index: other.palette_index,
            custom: ThemeWireColor {
                solid: other.custom.solid,
                select: other.custom.select,
                active: other.custom.active,
                flag: other.custom.flag,
            },
        }
    }

    pub fn hash(&self) -> u64 {
        if self.palette_index >= 0 {
            get_default_hash(self.palette_index)
        } else {
            let hash_solid = get_default_hash(self.custom.solid[0], self.custom.solid[1], self.custom.solid[2]);
            let hash_select = get_default_hash(
                self.custom.select[0], self.custom.select[1], self.custom.select[2],
            );
            let hash_active = get_default_hash(
                self.custom.active[0], self.custom.active[1], self.custom.active[2],
            );
            get_default_hash(hash_solid, hash_select, hash_active, self.custom.flag)
        }
    }
}

pub fn anim_bonecolor_posebone_get(pchanbone: &PChanBoneConst) -> &BoneColor {
    if pchanbone.pchan.color.palette_index == 0 {
        &pchanbone.bone.color
    } else {
        &pchanbone.pchan.color
    }
}
