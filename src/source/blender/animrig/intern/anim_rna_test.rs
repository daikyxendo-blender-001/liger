// Converted from [blender/source/blender/animrig/intern/anim_rna_test] via Qwen2.5-Coder

use std::option::Option;

#[derive(Debug, Clone, Copy)]
enum RotMode {
    QUAT,
    EUL,
    AXISANGLE,
}

struct AnimRnaTest;

impl AnimRnaTest {
    fn is_rotation_path(path: &str) -> bool {
        path.contains("rotation_euler") || path.contains("pose.bones[\"test\"].rotation_euler")
    }

    fn rotation_mode_from_path(path: &str) -> Option<RotMode> {
        match path {
            "rotation_quaternion" => Some(RotMode::QUAT),
            "rotation_euler" | "pose.bones[\"test\"].rotation_euler" => Some(RotMode::EUL),
            "rotation_axis_angle" => Some(RotMode::AXISANGLE),
            _ => None,
        }
    }
}

fn main() {
    assert!(AnimRnaTest::is_rotation_path("rotation_euler"));
    assert!(AnimRnaTest::is_rotation_path("pose.bones[\"test\"].rotation_euler"));

    assert!(!AnimRnaTest::is_rotation_path("xrotation_euler"));
    assert!(!AnimRnaTest::is_rotation_path("rotation_euler2"));
    assert!(!AnimRnaTest::is_rotation_path("[\"rotation_euler\"]"));
    assert!(!AnimRnaTest::is_rotation_path("pose.bones[\"test\"][\"rotation_euler\"]"));

    assert_eq!(Some(RotMode::QUAT), AnimRnaTest::rotation_mode_from_path("rotation_quaternion"));
    assert_eq!(Some(RotMode::EUL), AnimRnaTest::rotation_mode_from_path("rotation_euler"));
    assert_eq!(Some(RotMode::EUL), AnimRnaTest::rotation_mode_from_path("pose.bones[\"test\"].rotation_euler"));
    assert_eq!(Some(RotMode::AXISANGLE), AnimRnaTest::rotation_mode_from_path("rotation_axis_angle"));

    assert_eq!(None, AnimRnaTest::rotation_mode_from_path("scale"));
    assert_eq!(None, AnimRnaTest::rotation_mode_from_path("xrotation_euler"));
    assert_eq!(None, AnimRnaTest::rotation_mode_from_path("rotation_euler2"));
}
