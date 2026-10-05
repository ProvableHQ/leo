/*
seed = 987654321
min_height = 16

[case]
program = "test.aleo"
function = "test_vector_ops"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "test.aleo"
function = "test_else_if_storage"
input = ["true"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "test.aleo"
function = "test_else_if_storage"
input = ["false"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "test.aleo"
function = "test_skipped_empty_swap_remove"
input = ["true"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "test.aleo"
function = "test_false_else_if_storage"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

program test.aleo {
    storage vec: [u32];

    fn test_vector_ops() -> Final {
        return final {
            // Initialize: ensure empty
            assert(vec.len() == 0);

            // Push elements
            vec.push(10);
            vec.push(20);
            vec.push(30);
            assert(vec.len() == 3);

            // Get element at index 1 (should be 20)
            let val = vec.get(1).unwrap();
            assert(val == 20);

            // Set index 1 to 25
            vec.set(1, 25);
            let updated_val = vec.get(1).unwrap();
            assert(updated_val == 25);

            // Before: vec = [10, 25, 30]
            // After:  vec = [30, 25] and removed = 10
            let removed_non_last = vec.swap_remove(0);
            assert(removed_non_last == 10);
            assert(vec.len() == 2);

            // Verify that the last element (30) replaced index 0
            let first_elem = vec.get(0).unwrap();
            assert(first_elem == 30);
            let second_elem = vec.get(1).unwrap();
            assert(second_elem == 25);

            // Pop last element (should be 25 now)
            let popped = vec.pop().unwrap();
            assert(popped == 25);
            assert(vec.len() == 1);

            // Clear
            vec.clear();
            assert(vec.len() == 0);

            // Pop an empty vector
            assert(vec.pop() == none);

            vec.push(99);
            assert(vec.len() == 1);

            // Single-element swap_remove(0)
            // Should remove the only element, returning 99
            let removed_single = vec.swap_remove(0);
            assert(removed_single == 99);
            assert(vec.len() == 0);
        };
    }

    fn test_else_if_storage(skip: bool) -> Final {
        return final {
            vec.clear();
            vec.push(10u32);
            vec.push(20u32);

            if skip {
                assert_eq(vec.len(), 2u32);
            } else if vec.pop().unwrap() == 20u32 {
                assert_eq(vec.len(), 1u32);
            } else {
                assert(false);
            }
            assert_eq(vec.len(), skip ? 2u32 : 1u32);
            assert_eq(vec.get(0u32).unwrap(), 10u32);

            vec.clear();
            vec.push(10u32);
            vec.push(20u32);
            vec.push(30u32);

            if skip {
                assert_eq(vec.len(), 3u32);
            } else if vec.swap_remove(0u32) == 10u32 {
                assert_eq(vec.len(), 2u32);
                assert_eq(vec.get(0u32).unwrap(), 30u32);
            } else {
                assert(false);
            }
            assert_eq(vec.len(), skip ? 3u32 : 2u32);
            assert_eq(vec.get(1u32).unwrap(), 20u32);

            vec.clear();
            vec.push(10u32);
            vec.push(20u32);
            vec.push(30u32);

            if skip {
                assert_eq(vec.len(), 3u32);
            } else if vec.pop().unwrap() == 30u32 {
                assert_eq(vec.len(), 2u32);
            } else if vec.swap_remove(0u32) == 10u32 {
                assert(false);
            } else {
                assert(false);
            }
            assert_eq(vec.len(), skip ? 3u32 : 2u32);
            assert_eq(vec.get(0u32).unwrap(), 10u32);
            assert_eq(vec.get(1u32).unwrap(), 20u32);
        };
    }

    fn test_false_else_if_storage() -> Final {
        return final {
            vec.clear();
            vec.push(10u32);
            vec.push(20u32);
            vec.push(30u32);

            if vec.len() != 3u32 {
                assert(false);
            } else if vec.pop().unwrap() == 99u32 {
                assert(false);
            } else if vec.swap_remove(0u32) == 10u32 {
                assert_eq(vec.len(), 1u32);
                assert_eq(vec.get(0u32).unwrap(), 20u32);
            } else {
                assert(false);
            }
            assert_eq(vec.len(), 1u32);
            assert_eq(vec.get(0u32).unwrap(), 20u32);
        };
    }

    fn test_skipped_empty_swap_remove(skip: bool) -> Final {
        return final {
            vec.clear();
            if skip {
                assert_eq(vec.len(), 0u32);
            } else if vec.swap_remove(0u32) == 10u32 {
                assert(false);
            }
            assert_eq(vec.len(), 0u32);
        };
    }

    @noupgrade
    constructor() {}
}
