// FAIL: queries cannot write to mappings (read-only).
program test.aleo {
    mapping balances: address => u64;

    fn dummy(x: u32) -> u32 { return x; }

    query fn poison(addr: address, val: u64) -> u64 {
        balances.set(addr, val);
        return val;
    }

    @noupgrade
    constructor() {}
}
