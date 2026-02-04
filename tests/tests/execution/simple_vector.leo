/*
seed = 987654321
min_height = 16

[case]
program = "test.aleo"
function = "test_vector_ops"
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

    @noupgrade
    constructor() {}
}
