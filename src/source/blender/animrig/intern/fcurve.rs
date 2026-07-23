// Converted from [blender/source/blender/animrig/intern/fcurve] via Qwen2.5-Coder

This code snippet appears to be part of a larger system for handling and manipulating animation curves in Blender, a popular 3D modeling software. The functions provided here are used to insert keyframes into FCurves (which represent animation data), remove keyframes within specified ranges, and sample values from FCurves at regular intervals.

Here's a breakdown of what each function does:

1. `insert_vert_fcurve`: This function inserts a new keyframe into an F-Curve at the given frame with the specified value. It also handles interpolation based on neighboring keyframes and updates handle positions if necessary.

2. `sample_fcurve_segment`: Given an F-Curve, this function samples values at regular intervals between two frames (start_frame and start_frame + sample_rate * (sample_count - 1)).

3. `remove_fcurve_key_range`: This function removes keyframes from an F-Curve within a specified range based on the removal mode provided (either removing all keys outside the range, inside the range, or none at all).

4. `bake_fcurve`: This function bakes sampled values into the F-Curve by replacing existing keyframes with new ones based on the sampling parameters.

5. `bake_fcurve_segments`: This function identifies segments of selected keyframes in an F-Curve and samples values within those segments, then inserts new keyframes at these sampled points.

6. `fcurve_frame_has_keyframe`: This utility function checks if there is already a keyframe present on the specified frame for the given F-Curve.

The code uses various helper functions from Blender's internal API to manipulate FCurves and their keyframes, such as calculating handle positions (`BKE_fcurve_handles_recalc`), finding keyframes within a range (`BKE_fcurve_bezt_binarysearch_index`), and evaluating the curve at specific times (`evaluate_fcurve`).
