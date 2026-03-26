/*
[case]
program = "main.aleo"
function = "compute"
input = ["1u32", "2u32", "3u32", "4u32"]
*/

// Test: cross-library function call where the callee operates on a struct from another library.
// move_and_measure(Point{1,2}, 3, 4) -> translate -> Point{4,6} -> distance_sq -> 16+36 = 52.

// --- library: geo_lib --- //

struct Point {
    x: u32,
    y: u32,
}

fn translate(p: Point, dx: u32, dy: u32) -> Point {
    return Point { x: p.x + dx, y: p.y + dy };
}

fn distance_sq(p: Point) -> u32 {
    return p.x * p.x + p.y * p.y;
}

// --- Next Program --- //

// --- library: shape_lib --- //

fn move_and_measure(p: geo_lib::Point, dx: u32, dy: u32) -> u32 {
    let q: geo_lib::Point = geo_lib::translate(p, dx, dy);
    return geo_lib::distance_sq(q);
}

// --- Next Program --- //

program main.aleo {
    fn compute(px: u32, py: u32, dx: u32, dy: u32) -> u32 {
        let p: geo_lib::Point = geo_lib::Point { x: px, y: py };
        return shape_lib::move_and_measure(p, dx, dy);
    }

    @noupgrade
    constructor() {}
}
