// --- library: lib --- //

// FAIL: `view fn` is a program-only entry point and cannot appear in a library.
view fn helper(x: u64) -> u64 {
    return x + 1u64;
}

// --- Next Program --- //
program test.aleo {
    fn main(x: u32) -> u32 { return x; }

    @noupgrade
    constructor() {}
}
