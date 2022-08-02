// A (x, y) coordinate point.
circuit Point {
    x: u32,
    y: u32,
}

// Returns a new point.
function new(x: u32, y: u32) -> Point {
    return Point {x, y};
}

// Returns the sum of two points.
function sum(a: Point, b: Point) -> Point {
    return Point {
        x: a.x + b.x,
        y: a.y + b.y,
    };
}