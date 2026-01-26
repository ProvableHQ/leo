/*
seed = 987654321
min_height = 16

[case]
program = "test_oob.aleo"
function = "oob_get"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "test_oob.aleo"
function = "oob_set"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "test_oob.aleo"
function = "oob_swap_remove"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"

[case]
program = "test_oob.aleo"
function = "oob_pop"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

program test_oob.aleo {
    storage vec: [u32];

    // OOB get: should return None
    fn oob_get() -> Final {
        return final {
            vec.clear();
            assert(vec.len() == 0u32);

            let result = vec.get(0u32);
            assert(result == none);
        };
    }

    // OOB set: should panic (transition rejected)
    fn oob_set() -> Final {
        return final {
            vec.clear();
            assert(vec.len() == 0u32);

            vec.set(0u32, 123u32);
        };
    }

    // OOB swap_remove: should panic (transition rejected)
    fn oob_swap_remove() -> Final {
        return final {
            vec.clear();
            assert(vec.len() == 0u32);

            vec.swap_remove(0u32);
        };
    }

    // Pop empty vector: should return None
    fn oob_pop() -> Final {
        return final {
            vec.clear();
            assert(vec.len() == 0u32);

            let popped = vec.pop();
            assert(popped == none);
        };
    }

    @noupgrade
    constructor() {}
}
