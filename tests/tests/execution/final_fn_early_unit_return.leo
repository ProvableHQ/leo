/*
seed = 123456789
min_height = 16

[case]
program = "final_fn_early_unit_return.aleo"
function = "f"
input = ["0u64"]
[case]
program = "final_fn_early_unit_return.aleo"
function = "f"
input = ["5u64"]
*/

// An early bare `return;` in a `final fn` must skip the rest of the helper body
// but still run the statements after the callsite.
export final fn record_nonzero(amount: u64) {
    if amount == 0u64 {
        return;
    }
    // If the early return above ever fell through, the zero case would halt here.
    assert_neq(amount, 0u64);
    Mapping::set(store, 1u8, amount);
}

program final_fn_early_unit_return.aleo {
    mapping store: u8 => u64;

    fn f(amount: u64) -> Final {
        return final {
            record_nonzero(amount);
            Mapping::set(store, 0u8, amount + 1u64);
        };
    }

    @noupgrade
    constructor() {}
}
