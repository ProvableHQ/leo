/*
[case]
program = "submodule_struct.aleo"
function = "compute"
input = ["3u32", "4u32"]
*/

// --- library: geom_lib --- //

// --- Next Module: types.leo --- //
struct Vec2 {
    x: u32,
    y: u32,
}

fn norm_sq(v: Vec2) -> u32 {
    return v.x * v.x + v.y * v.y;
}

// --- Next Program --- //

program submodule_struct.aleo {
    fn compute(x: u32, y: u32) -> u32 {
        let v: geom_lib::types::Vec2 = geom_lib::types::Vec2 { x, y };
        return geom_lib::types::norm_sq(v);  // 3*3 + 4*4 = 25
    }

    @noupgrade
    constructor() {}
}
