// Submodule: geometric types and helpers.

const UNIT: u32 = 1u32;

struct Point {
    x: u32,
    y: u32,
}

struct Rect {
    top_left: Point,
    bottom_right: Point,
}

fn area(r: Rect) -> u32 {
    return (r.bottom_right.x - r.top_left.x) * (r.bottom_right.y - r.top_left.y);
}

fn unit_rect() -> Rect {
    return Rect {
        top_left: Point { x: 0u32, y: 0u32 },
        bottom_right: Point { x: UNIT, y: UNIT },
    };
}
