// A (x, y) coordinate point
circuit Point {
    x: u32,
    y: u32,
}

function new(x: u32, y: u32) -> Point {
    return Point {x, y};
}

function add(a: Point, b: Point) -> Point {
    return Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}