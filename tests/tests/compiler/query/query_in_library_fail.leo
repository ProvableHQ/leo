// --- Next Library: lib --- //
// FAIL: queries are program-only; libraries cannot declare them.
query fn helper(x: u64) -> u64 {
    return x + 1u64;
}

// --- Next Program --- //
program test.aleo {
    fn main(x: u32) -> u32 { return x; }

    @noupgrade
    constructor() {}
}
