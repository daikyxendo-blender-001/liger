// Converted from [blender/source/blender/animrig/intern/keyframing] via Qwen2.5-Coder

The provided code snippet is a part of a larger system for keyframing in Blender, specifically within the `blender/animrig` module. The functions `insert_key_layered_action` and `insert_keyframes` are designed to handle the process of inserting keyframes into an animation action based on various parameters such as property paths, frames, values, and flags.

Here's a breakdown of what each function does:

1. **Function: `insert_key_layered_action`**
   - This function is responsible for inserting keyframes into a specific layer of an action.
   - It takes several parameters including the Blender main data block (`bmain`), the action to modify, the layer and slot within the action, the property to keyframe, channel group (if any), RNA path, frame, values, insert key flags, keyframe settings, and a keying mask.
   - The function iterates over each value provided in the `values` span. For each value, it checks if the corresponding index in the `keying_mask` is set to allow keying. If so, it constructs a `KeyInsertData` object containing the frame and value, and calls another function `insert_key_layer` to actually insert the keyframe.
   - The function returns a `CombinedKeyingResult` object that summarizes the results of all key insertion attempts.

2. **Function: `insert_keyframes`**
   - This function is designed to handle the insertion of keyframes for multiple properties in an action.
   - It takes parameters such as the Blender main data block (`bmain`), a pointer to the RNA structure, an optional channel group, a span of RNA paths, an optional scene frame, an animation evaluation context, a keyframe type, and insert key flags.
   - The function first ensures that the ID is animatable and that there is an action available. It then prepares the action layer for keying.
   - For each RNA path provided, it resolves the property and retrieves its values. It then remaps these values based on NLA (Non-Linear Animation) settings if applicable.
   - The function constructs a `KeyframeSettings` object based on the insert key flags and key type.
   - It then calls `insert_key_layered_action` for each RNA path to insert keyframes into the action.
   - Finally, it updates the action and returns a `CombinedKeyingResult` summarizing all operations.

### Key Points:
- **Error Handling**: Both functions include error handling to manage cases where properties cannot be resolved, actions cannot be created, or IDs are not editable.
- **NLA Remapping**: The `insert_keyframes` function includes logic for remapping keyframe values based on NLA settings, which is crucial for maintaining the integrity of animated data when dealing with non-linear animations.
- **Keying Flags**: The functions use various flags to control the behavior of key insertion, such as whether to replace existing keys or only insert if available.

### Usage:
These functions are typically called from within Blender's user interface or scripting environment to allow users to manually keyframe properties of objects in an animation. They provide a robust framework for handling complex keying operations, including support for non-linear animations and multiple property paths.

This code is part of the larger Blender system for managing and manipulating animation data, ensuring that it can handle a wide range of use cases while maintaining performance and correctness.
