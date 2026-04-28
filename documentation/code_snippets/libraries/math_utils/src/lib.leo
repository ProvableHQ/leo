// ANCHOR: basics
/// The maximum value representable by a u32.
const MAX_U32: u32 = 4294967295u32;

/// A 2-D point with integer coordinates.
struct Point {
    x: i32,
    y: i32,
}

/// Returns the absolute value of a signed 32-bit integer.
fn abs(x: i32) -> i32 {
    return x >= 0i32 ? x : 0i32 - x;
}

/// Returns the Manhattan distance between two points.
fn manhattan(a: Point, b: Point) -> u32 {
    let dx: i32 = abs(a.x - b.x);
    let dy: i32 = abs(a.y - b.y);
    return dx as u32 + dy as u32;
}
// ANCHOR_END: basics

// ANCHOR: clamp
/// Clamps `value` to the range [0, MAX].
fn clamp::[MAX: u32](value: u32) -> u32 {
    return value > MAX ? MAX : value;
}
// ANCHOR_END: clamp
