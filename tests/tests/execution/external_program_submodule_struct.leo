/*
[case]
program = "parent.aleo"
function = "sum_local_struct"
input = ["3u32", "4u32"]

[case]
program = "parent.aleo"
function = "sum_via_child"
input = ["5u32", "6u32"]
*/

// Test: end-to-end execution with structs from submodules of an imported .aleo program.

program child.aleo {
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

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // Construct a submodule struct locally and access its fields.
    fn sum_local_struct(x: u32, y: u32) -> u32 {
        let p: child.aleo::types::Point = child.aleo::types::Point { x: x, y: y };
        return p.x + p.y;
    }

    // Call a child entry fn that returns a submodule struct, then access fields.
    fn sum_via_child(x: u32, y: u32) -> u32 {
        let p: child.aleo::types::Point = child.aleo::make_point_entry(x, y);
        return p.x + p.y;
    }

    @noupgrade
    constructor() {}
}
