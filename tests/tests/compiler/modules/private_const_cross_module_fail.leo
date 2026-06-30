// Test: a const in a submodule is not visible to a sibling site without `export`.

program test.aleo {
    fn get() -> u32 {
        return util::HIDDEN;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: util.leo --- //

const HIDDEN: u32 = 42u32;
