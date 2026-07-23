// Test: accessing a non-`export` function from a sibling module of the same program
// fails with an inaccessible-item error. Same-file access works (no `export` needed there).

program private_xmod.aleo {
    fn main() -> u32 {
        // Cross-module access to `util::secret` is rejected because `secret` is not `export`.
        return util::secret();
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: util.leo --- //

// Visible inside `util.leo` itself but not from other modules.
fn secret() -> u32 {
    return 7u32;
}

// A wrapper that demonstrates same-file access does not need `export`.
export fn relay() -> u32 {
    return secret();
}
