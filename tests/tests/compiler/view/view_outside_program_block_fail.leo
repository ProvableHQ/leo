// FAIL: `view fn` is a program-only entry point and cannot appear at file top level
// (outside any `program { ... }` block).
view fn helper(x: u64) -> u64 {
    return x + 1u64;
}

program test.aleo {
    fn main(x: u32) -> u32 { return x; }

    @noupgrade
    constructor() {}
}
