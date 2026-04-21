// Calls a file-level `fn` (a `Variant::Fn` closure) defined outside the program block of an
// imported Leo program. The single call site trips the `call_count == 1` heuristic, so the
// function is inlined and the regenerated child stub drops `foo` as dead code. See
// `call_external_top_level_fn_multi.leo` for the multi-call case where the closure is kept
// and invoked via `call child.aleo/foo`.

program child.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

fn foo() -> u32 {
    return 42u32;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn main() -> u32 {
        return child.aleo::foo();
    }

    @noupgrade
    constructor() {}
}
