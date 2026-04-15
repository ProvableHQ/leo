// Negative test: accessing a struct from a submodule that does not exist should
// produce a type-checker error.

program child.aleo {
    fn dummy() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: types.leo --- //

struct Point {
    x: u32,
    y: u32,
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // The module `ghost` does not exist in child.aleo — should be a type error.
    fn bad_type(p: child.aleo::ghost::Point) -> u32 {
        return p.x + p.y;
    }

    @noupgrade
    constructor() {}
}
