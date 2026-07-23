// Converted from [blender/source/blender/animrig/intern/armature] via Qwen2.5-Coder

use std::collections::VecDeque;
use std::ptr;

#[derive(Debug, Clone, Copy)]
struct Object {
    pose: *mut Pose,
}

#[derive(Debug, Clone, Copy)]
struct bPoseChannel {
    bone_get: unsafe fn(&Object) -> *const Bone,
    name: *const u8,
}

#[derive(Debug, Clone, Copy)]
struct Bone {
    childbase: Vec<Bone>,
}

#[derive(Debug, Clone, Copy)]
struct Pose {
    channels: Vec<bPoseChannel>,
}

type Callback = dyn FnMut(bPoseChannel);

fn pose_bone_descendent_iterator(pose_ob: &Object, pchan: bPoseChannel, callback: Callback) {
    unsafe {
        BKE_pose_channels_hash_ensure(pose_ob.pose);
    }

    let mut descendants = VecDeque::new();
    descendants.push_back(&pchan);

    while let Some(descendant) = descendants.pop_front() {
        callback(*descendant);
        let descendant_bone = unsafe { (descendant.bone_get)(pose_ob) };
        for child_bone in &descendant_bone.childbase {
            if let Some(child_pose_bone) = unsafe { BKE_pose_channel_find_name(pose_ob.pose, child_bone.name) } {
                descendants.push_back(child_pose_bone);
            }
        }
    }
}

fn pose_depth_iterator_recursive(
    pose_ob: &Object,
    pchanbone: (bPoseChannel, *const Bone),
    callback: Callback,
) -> bool {
    if !callback(pchanbone.0) {
        return false;
    }

    let mut success = true;
    for child_bone in unsafe { (*pchanbone.1).childbase.iter() } {
        if let Some(child_pose_bone) = unsafe { BKE_pose_channel_find_name(pose_ob.pose, child_bone.name) } {
            success &= pose_depth_iterator_recursive(pose_ob, (child_pose_bone, child_bone), callback);
        }
    }

    success
}

fn pose_bone_descendent_depth_iterator(
    pose_ob: &Object,
    pchan: bPoseChannel,
    callback: Callback,
) -> bool {
    unsafe {
        BKE_pose_channels_hash_ensure(pose_ob.pose);
    }

    pose_depth_iterator_recursive(pose_ob, (pchan, unsafe { (pchan.bone_get)(pose_ob) }), callback)
}

extern "C" {
    fn BKE_pose_channels_hash_ensure(pose: *mut Pose);
    fn BKE_pose_channel_find_name(pose: *mut Pose, name: *const u8) -> Option<bPoseChannel>;
}
