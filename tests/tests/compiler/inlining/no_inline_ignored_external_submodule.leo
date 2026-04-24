// Test: `@no_inline` on a submodule function of an external Leo program is
// ignored.  Submodule functions never exist in Aleo bytecode, so the inliner
// must force-inline them regardless of `@no_inline`.  A warning is emitted.

program child.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: math.leo --- //

@no_inline
fn helper(x: u32) -> u32 {
    return x + 1u32;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn call_helper(x: u32) -> u32 {
        return child.aleo::math::helper(x);
    }

    @noupgrade
    constructor() {}
}
