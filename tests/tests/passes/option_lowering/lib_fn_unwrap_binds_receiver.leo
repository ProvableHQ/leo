// Test: a library function returning `u32?` is called from the main program;
// the cross-program call result is bound to `$opt$N` before `.is_some` and
// `.val` accesses are synthesized.

// --- library: opt_lib --- //

fn maybe(x: u32) -> u32? {
    return x;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32, d: u32) -> u32 {
        return opt_lib::maybe(x).unwrap_or(d);
    }

    @noupgrade
    constructor() {}
}
