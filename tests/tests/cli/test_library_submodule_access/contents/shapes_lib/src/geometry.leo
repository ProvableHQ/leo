// Submodule: geometric types and helpers.

export const UNIT: u32 = 1u32;

export struct Point {
    x: u32,
    y: u32,
}

export struct Rect {
    top_left: Point,
    bottom_right: Point,
}

export fn area(r: Rect) -> u32 {
    return (r.bottom_right.x - r.top_left.x) * (r.bottom_right.y - r.top_left.y);
}

export fn unit_rect() -> Rect {
    return Rect {
        top_left: Point { x: 0u32, y: 0u32 },
        bottom_right: Point { x: UNIT, y: UNIT },
    };
}
