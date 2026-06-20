// Test: a struct in a submodule is not visible to a sibling site without `export`.
// The struct is referenced from inside the program block (module path `[]`) while
// the struct is declared in submodule `util` (module path `[util]`).

program test.aleo {
    fn make(x: u32, y: u32) -> util::Hidden {
        return util::Hidden { x, y };
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: util.leo --- //

struct Hidden {
    x: u32,
    y: u32,
}
