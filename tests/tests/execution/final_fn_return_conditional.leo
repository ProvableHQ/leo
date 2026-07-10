/*
seed = 123456789
min_height = 16

[case]
program = "final_fn_return_conditional.aleo"
function = "f"
input = ["500u64"]
[case]
program = "final_fn_return_conditional.aleo"
function = "f"
input = ["2000u64"]
*/

// Drives a value-returning `final fn` with a conditional return through both branches.
// The statements after the callsite must execute on each path with the path's value.
export final fn capped(amount: u64) -> u64 {
    if amount > 1000u64 {
        return 1000u64;
    }
    return amount;
}

program final_fn_return_conditional.aleo {
    mapping store: u8 => u64;

    fn f(amount: u64) -> Final {
        return final {
            let credited: u64 = capped(amount);
            Mapping::set(store, 0u8, credited);
            assert(credited <= 1000u64);
        };
    }

    @noupgrade
    constructor() {}
}
