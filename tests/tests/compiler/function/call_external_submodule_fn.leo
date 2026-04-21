// Calls a submodule function (`child.aleo::helpers::bump`) from an imported Leo program.
// Submodule functions don't exist in Aleo bytecode — every cross-program call site must be
// inlined, and the regenerated child stub must not emit the submodule function as a closure.

program child.aleo {
    fn main(public a: u32) -> u32 {
        return a;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: helpers.leo --- //

fn bump(x: u32) -> u32 {
    return x + 1u32;
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn main(x: u32) -> u32 {
        return child.aleo::helpers::bump(x);
    }

    @noupgrade
    constructor() {}
}
