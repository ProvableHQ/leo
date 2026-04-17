// Generic top-level `fn` in an external Leo program, called cross-program with a const
// argument. The single call site trips the `call_count == 1` heuristic, so monomorphization
// produces an instance that is then inlined at the call site and the regenerated child stub
// drops it as dead code. See `call_external_top_level_generic_fn_multi.leo` for the multi-call
// case where the monomorphized closure is kept and invoked via `call child.aleo/...`.

program child.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

fn add_const::[N: u32](x: u32) -> u32 {
    return x + N;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn main(x: u32) -> u32 {
        return child.aleo::add_const::[10u32](x);
    }

    @noupgrade
    constructor() {}
}
