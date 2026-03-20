// Test: cross-library function call where both libraries operate on the same struct type.
//
// geo_lib defines Point and translate(). shape_lib defines pipeline() which calls
// geo_lib::translate() and returns a geo_lib::Point. The program calls shape_lib::pipeline().
// This exercises the cross-library call graph where the function argument and return types
// cross the library boundary.

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
