program child.aleo {
    @noupgrade
    constructor() {}

    fn main(public a: u32, b: u32) -> u32 {
        return a + b;
    }
}

// File-level `fn` outside the program block. This is a `Variant::Fn` closure.
// Aleo `call` supports cross-program closure targets, but the inliner's
// conditional heuristics (single call site, no args) still fire here, so this
// ends up inlined at the call site.
fn foo() -> u32 {
    return 42u32;
}
