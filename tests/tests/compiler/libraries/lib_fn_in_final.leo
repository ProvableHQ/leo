// Test: a library function called inside a final {} block (on-chain execution context).

// --- library: math_lib --- //

fn add(a: u64, b: u64) -> u64 {
    return a + b;
}

// --- Next Program --- //

program main.aleo {
    mapping totals: address => u64;

    fn accumulate(owner: address, amount: u64) -> Final {
        return final {
            let prev: u64 = Mapping::get_or_use(totals, owner, 0u64);
            Mapping::set(totals, owner, math_lib::add(prev, amount));
        };
    }

    @noupgrade
    constructor() {}
}
