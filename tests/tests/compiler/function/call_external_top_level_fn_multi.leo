// A top-level `fn` (a `Variant::Fn` closure) defined outside the program block of an imported
// Leo program, invoked from two distinct call sites in the parent. Because the inliner's
// conditional heuristics don't fire (call_count > 1, has args, non-empty types), the closure
// is emitted in the regenerated child stub and the parent emits `call child.aleo/foo` at each
// site — Aleo `call` supports cross-program closure targets.

program child.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

fn foo(x: u32) -> u32 {
    return x * x;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn main(x: u32, y: u32) -> u32 {
        let a: u32 = child.aleo::foo(x);
        let b: u32 = child.aleo::foo(y);
        return a + b;
    }

    @noupgrade
    constructor() {}
}
