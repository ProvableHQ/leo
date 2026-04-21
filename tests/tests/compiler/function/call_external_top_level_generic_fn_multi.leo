// Generic top-level `fn` in an external Leo program, called cross-program with the same const
// argument at two distinct sites. Monomorphization specializes `add_const::[10u32]` once; the
// inliner's conditional heuristics don't fire (call_count > 1, has args), so the specialization
// is emitted as a closure in the regenerated child stub and the parent emits `call` at each
// site.

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
    fn main(x: u32, y: u32) -> u32 {
        let a: u32 = child.aleo::add_const::[10u32](x);
        let b: u32 = child.aleo::add_const::[10u32](y);
        return a + b;
    }

    @noupgrade
    constructor() {}
}
