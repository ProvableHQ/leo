// Test: referencing a non-`export` struct in another compilation unit fails.

// --- library: types_lib --- //

struct Hidden {
    x: u32,
    y: u32,
}

// --- Next Program --- //

program main.aleo {
    fn make(x: u32, y: u32) -> types_lib::Hidden {
        return types_lib::Hidden { x, y };
    }

    @noupgrade
    constructor() {}
}
