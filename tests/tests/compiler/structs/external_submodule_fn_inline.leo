// Positive test: calling a function from a submodule of an external program is
// supported — the submodule function is inlined at the call site.

program child.aleo {
    fn top_fn(x: u32) -> u32 {
        return x + 1u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: utils.leo --- //

fn _helper(x: u32) -> u32 {
    return x * 2u32;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn call_helper(x: u32) -> u32 {
        return child.aleo::utils::_helper(x);
    }

    @noupgrade
    constructor() {}
}
