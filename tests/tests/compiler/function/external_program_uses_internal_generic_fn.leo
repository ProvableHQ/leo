// The current program doesn't call an imported program's generic top-level `fn` — but the
// imported program calls it internally. Monomorphization must still visit that internal call
// during the current compilation so the const-generic reference is substituted; otherwise a
// dangling `N` survives into code generation and panics.

program child.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c + foo::[42u32]();
    }

    @noupgrade
    constructor() {}
}

fn foo::[N: u32]() -> u32 {
    return N;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn main() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
