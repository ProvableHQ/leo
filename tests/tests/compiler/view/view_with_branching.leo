// Branching inside a view lowers to Aleo `branch` instructions, not flattened to ternaries.
program test.aleo {
    mapping balances: address => u64;

    fn dummy(x: u32) -> u32 {
        return x;
    }

    view fn capped_balance(addr: address, cap: u64) -> u64 {
        let raw: u64 = balances.get_or_use(addr, 0u64);
        if raw > cap {
            return cap;
        } else {
            return raw;
        }
    }

    @noupgrade
    constructor() {}
}
