// Test: intra-library call chain involving library structs.
//
// `pipeline` calls `translate` (same library) and passes the returned `Point` to
// `distance_sq` (same library). This exercises the combined intra-library call graph
// and struct graph: functions reference each other AND share the same library-local struct.

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
