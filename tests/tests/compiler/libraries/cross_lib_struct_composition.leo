// Test: a struct in library B uses a non-generic struct from library A as a field type.
//
// Verifies that the composite dependency graph contains the cross-library edge
// shape_lib/Rect -> geo_lib/Point and that post-order traversal processes Point
// before Rect, so that geo_lib::Point is emitted in the bytecode before shape_lib::Rect.
//
// Dependency graph:
//   geo_lib    (library: concrete struct Point)
//   shape_lib  (library: struct Rect containing geo_lib::Point fields)
//   main.aleo  (program: uses shape_lib::Rect)

// --- library: geo_lib --- //

struct Point {
    x: u32,
    y: u32,
}

// --- Next Program --- //

// --- library: shape_lib --- //

struct Rect {
    top_left: geo_lib::Point,
    bottom_right: geo_lib::Point,
}

// --- Next Program --- //

program main.aleo {
    fn make_rect(x1: u32, y1: u32, x2: u32, y2: u32) -> shape_lib::Rect {
        return shape_lib::Rect {
            top_left: geo_lib::Point { x: x1, y: y1 },
            bottom_right: geo_lib::Point { x: x2, y: y2 },
        };
    }

    fn width(r: shape_lib::Rect) -> u32 {
        return r.bottom_right.x - r.top_left.x;
    }

    @noupgrade
    constructor() {}
}
