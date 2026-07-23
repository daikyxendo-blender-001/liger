// Converted from [blender/source/blender/animrig/intern/bone_collections] via Qwen2.5-Coder

This code snippet is part of a larger system for managing bone collections in an armature (a structure used to represent skeletal animation) within Blender, a popular 3D modeling and animation software. The functions provided here are designed to handle various operations related to bone collections, including copying, moving, rotating, and freeing them.

1. **Copying Bone Collections**: The `ANIM_bonecoll_array_copy_no_membership` function creates a deep copy of an array of bone collections without copying their membership (i.e., the bones they contain). This is useful for creating undo states in edit mode where you need to revert changes to the bone collection structure.

2. **Freeing Bone Collections**: The `ANIM_bonecoll_array_free` function properly frees all memory associated with an array of bone collections, including their properties and membership. This ensures that there are no memory leaks when undo operations are performed.

3. **Internal Functions for Manipulating Bone Collections**:
   - `bonecolls_rotate_block`: Rotates a block of bone collections within the armature's collection array by one position in either direction.
   - `bonecolls_move_to_index`: Moves a single bone collection to a new index within the array.
   - `bonecolls_find_index_near`: Finds the nearest index of a given bone collection to another specified index.
   - `bonecoll_unassign_and_free`: Removes all bone memberships from a bone collection and then frees it.

4. **Debugging**: The `bonecolls_debug_list` function provides a simple way to print out the current state of bone collections in an armature, which can be useful for debugging purposes.

These functions are essential for maintaining the integrity and functionality of bone collections during various operations within Blender's armature editing system.
