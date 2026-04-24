// Test: option lowering of `.unwrap_or(...)` on a receiver that is a submodule
// function call. The call subtree is bound to `$opt$N` before the two member
// accesses are synthesized, exactly as for non-library calls.

// --- library: opt_lib --- //

// --- Next Module: ops.leo --- //
fn maybe(x: u32) -> u32? {
    return x;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32, d: u32) -> u32 {
        return opt_lib::ops::maybe(x).unwrap_or(d);
    }

    @noupgrade
    constructor() {}
}
