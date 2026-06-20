// Test: a struct in a submodule is not visible from a sibling submodule without `export`.

program test.aleo {
    fn entry(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: parent.leo --- //

export fn make() -> helpers::Hidden {
    return helpers::Hidden { x: 1u32, y: 2u32 };
}

// --- Next Module: parent/helpers.leo --- //

struct Hidden {
    x: u32,
    y: u32,
}
