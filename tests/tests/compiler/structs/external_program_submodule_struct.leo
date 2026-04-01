// Test: accessing structs from submodules of an imported .aleo program.

program child.aleo {
    fn get_val() -> u32 {
        return 1u32;
    }

    fn make_point_entry(x: u32, y: u32) -> types::Point {
        return types::Point { x: x, y: y };
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: types.leo --- //

struct Point {
    x: u32,
    y: u32,
}

fn make_point(x: u32, y: u32) -> Point {
    return Point { x: x, y: y };
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // Struct type annotation from submodule.
    fn use_type(p: child.aleo::types::Point) -> u32 {
        return p.x + p.y;
    }

    // Struct literal construction from submodule.
    fn make_struct(x: u32, y: u32) -> child.aleo::types::Point {
        return child.aleo::types::Point { x: x, y: y };
    }

    // Call a top-level entry fn from child that returns a submodule struct type.
    fn call_child_entry(x: u32, y: u32) -> child.aleo::types::Point {
        return child.aleo::make_point_entry(x, y);
    }

    @noupgrade
    constructor() {}
}
