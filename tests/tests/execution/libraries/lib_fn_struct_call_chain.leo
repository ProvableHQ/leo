/*
[case]
program = "main.aleo"
function = "compute"
input = ["1u32", "2u32", "3u32", "4u32"]
*/

// Test: intra-library call chain involving library structs.
// pipeline(Point{1,2}, 3, 4) -> translate -> Point{4,6} -> distance_sq -> 16+36 = 52.

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

fn pipeline(p: Point, dx: u32, dy: u32) -> u32 {
    return distance_sq(translate(p, dx, dy));
}

// --- Next Program --- //

program main.aleo {
    fn compute(px: u32, py: u32, dx: u32, dy: u32) -> u32 {
        let p: geo_lib::Point = geo_lib::Point { x: px, y: py };
        return geo_lib::pipeline(p, dx, dy);
    }

    @noupgrade
    constructor() {}
}
