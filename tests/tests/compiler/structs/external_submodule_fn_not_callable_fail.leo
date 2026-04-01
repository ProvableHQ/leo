// Negative test: calling a function from a submodule of an external program is
// not supported — submodule functions are not exported to Aleo bytecode.

program child.aleo {
    fn top_fn(x: u32) -> u32 {
        return x + 1u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: utils.leo --- //

fn helper(x: u32) -> u32 {
    return x * 2u32;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn bad_call(x: u32) -> u32 {
        return child.aleo::utils::helper(x);
    }

    @noupgrade
    constructor() {}
}
