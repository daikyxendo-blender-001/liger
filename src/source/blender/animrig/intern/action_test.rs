// Converted from [blender/source/blender/animrig/intern/action_test] via Qwen2.5-Coder

This code snippet is a unit test for the `Channelbag` class in Blender's animation rigging system. The `Channelbag` class manages F-Curves (which represent keyframes in animations) and their associated channel groups.

The test suite includes several tests:

1. **Channelbag Initialization**: Tests that the Channelbag can be initialized with a default constructor.
2. **F-Curve Creation**: Verifies that F-Curves can be created and added to the Channelbag.
3. **F-Curve Assignment to Group**: Checks if an F-Curve can be assigned to a channel group correctly.
4. **F-Curve Ungrouping**: Ensures that an F-Curve can be ungrouped from its current channel group.
5. **F-Curve Movement Between Actions**: Tests the functionality of moving an F-Curve from one Action (animation) to another.

The `ActionFCurveMoveTest` class is a specialized test fixture for testing F-Curve movement between actions, inheriting from `BlenderGTestBase`. It sets up and tears down a Blender main data block (`Main`) for each test case.

The actual tests are implemented using Google Test macros like `TEST_F`, `ASSERT_EQ`, and `EXPECT_TRUE` to validate the behavior of the Channelbag class methods.
