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

    async transition test_vector_ops() -> Future {
        return async {
            // Initialize: ensure empty
            assert(vec.len() == 0u32);

            // Push elements
            vec.push(10u32);
            vec.push(20u32);
            vec.push(30u32);
            assert(vec.len() == 3u32);

            // Get element at index 1 (should be 20)
            let val = vec.get(1u32).unwrap();
            assert(val == 20u32);

            // Set index 1 to 25
            vec.set(1u32, 25u32);
            let updated_val = vec.get(1u32).unwrap();
            assert(updated_val == 25u32);

            // Swap remove last element (should be 30)
            let removed = vec.swap_remove(2u32).unwrap();
            assert(removed == 30u32);
            assert(vec.len() == 2u32);

            // Pop last element (should be 25 now)
            let popped = vec.pop().unwrap();
            assert(popped == 25u32);
            assert(vec.len() == 1u32);

            // Clear
            vec.clear();
            assert(vec.len() == 0u32);
        };
    }

    @noupgrade
    async constructor() {}
}
