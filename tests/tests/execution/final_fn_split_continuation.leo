/*
seed = 123456789
min_height = 16

[case]
program = "final_fn_split_continuation.aleo"
function = "f"
input = ["500u64"]
[case]
program = "final_fn_split_continuation.aleo"
function = "f"
input = ["2000u64"]
[case]
program = "final_fn_split_continuation.aleo"
function = "g"
input = ["500u64"]
[case]
program = "final_fn_split_continuation.aleo"
function = "g"
input = ["2000u64"]
*/

// Drives both continuation-split shapes through both helper branches: `f` forces
// def-closure duplication (the tag write reads a prefix-defined name), `g` keeps a
// shared suffix that runs after the rejoin. A wrong cut surfaces as a runtime halt
// on an unwritten register.
export final fn capped(amount: u64) -> u64 {
    if amount > 1000u64 {
        return 1000u64;
    }
    return amount;
}

program final_fn_split_continuation.aleo {
    mapping store: u8 => u64;

    fn f(amount: u64) -> Final {
        return final {
            let credited: u64 = capped(amount);
            let tag: u64 = amount + 7u64;
            Mapping::set(store, 0u8, credited);
            Mapping::set(store, 1u8, tag);
        };
    }

    fn g(amount: u64) -> Final {
        return final {
            let credited: u64 = capped(amount);
            Mapping::set(store, 2u8, credited);
            Mapping::set(store, 3u8, amount);
        };
    }

    @noupgrade
    constructor() {}
}
