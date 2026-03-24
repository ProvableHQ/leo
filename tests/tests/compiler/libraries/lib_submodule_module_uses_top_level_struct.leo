// Test: a submodule function takes a struct defined at the library top level as a parameter.

// --- library: shape_lib --- //

struct Point { x: u32, y: u32 }

// --- Next Module: ops.leo --- //

fn norm_sq(p: shape_lib::Point) -> u32 {
    return p.x * p.x + p.y * p.y;
}

// --- Next Program --- //

program top_struct_usage.aleo {
    fn main() -> u32 {
        let p: shape_lib::Point = shape_lib::Point { x: 3u32, y: 4u32 };
        return shape_lib::ops::norm_sq(p);  // 3*3 + 4*4 = 25
    }

    @noupgrade
    constructor() {}
}
