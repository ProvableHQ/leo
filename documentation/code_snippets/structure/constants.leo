// ANCHOR: scopes
const MAX: u64 = 100u64;           // global constant

program constants_demo.aleo {
    const MULTIPLIER: u64 = 2u64;  // program-scope constant

    fn compute(x: u64) -> u64 {
        const OFFSET: u64 = 5u64;  // local constant
        return x * MULTIPLIER + OFFSET;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: scopes

// ANCHOR: expressions
const BASE: u32 = 10u32;
const LIMIT: u32 = BASE * 5u32;   // expression over constants
const PAIR: (u32, bool) = (LIMIT, true);   // tuple constant
// ANCHOR_END: expressions
