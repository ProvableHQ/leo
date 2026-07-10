// Test: a const in a submodule is not visible from a sibling submodule without `export`.

program test.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: parent.leo --- //

export fn get() -> u32 {
    return helpers::HIDDEN;
}

// --- Next Module: parent/helpers.leo --- //

const HIDDEN: u32 = 42u32;
