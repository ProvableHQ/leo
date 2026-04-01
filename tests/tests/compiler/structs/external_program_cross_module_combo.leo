// Test: accessing types from two different submodules of the same external program
// in a single function, and a struct that contains another struct as a field
// (struct-depends-on-struct), both from the same submodule.

program child.aleo {
    fn get_origin() -> shapes::Point {
        return shapes::Point { x: 0u32, y: 0u32 };
    }

    fn make_segment(x0: u32, y0: u32, x1: u32, y1: u32) -> shapes::Segment {
        let a: shapes::Point = shapes::Point { x: x0, y: y0 };
        let b: shapes::Point = shapes::Point { x: x1, y: y1 };
        return shapes::Segment { start: a, end: b };
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: shapes.leo --- //

struct Point {
    x: u32,
    y: u32,
}

// Segment depends on Point — struct field typed as another struct in same submodule.
struct Segment {
    start: Point,
    end: Point,
}

// --- Next Module: transforms.leo --- //

struct Scaled {
    origin: u32,
    factor: u32,
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // Cross-module combo: parameters from two different submodules in one function.
    fn combine(p: child.aleo::shapes::Point, s: child.aleo::transforms::Scaled) -> u32 {
        return p.x + p.y + s.origin + s.factor;
    }

    // Struct-depends-on-struct: Segment contains two Point fields.
    fn segment_length(seg: child.aleo::shapes::Segment) -> u32 {
        return (seg.end.x - seg.start.x) + (seg.end.y - seg.start.y);
    }

    // Build a Segment using the submodule struct literal syntax.
    fn make_and_measure(x0: u32, y0: u32, x1: u32, y1: u32) -> u32 {
        let seg: child.aleo::shapes::Segment = child.aleo::shapes::Segment {
            start: child.aleo::shapes::Point { x: x0, y: y0 },
            end: child.aleo::shapes::Point { x: x1, y: y1 },
        };
        return (seg.end.x - seg.start.x) + (seg.end.y - seg.start.y);
    }

    // Mix: combine cross-module and struct-depends-on-struct in one call chain.
    fn full_combo(seg: child.aleo::shapes::Segment, s: child.aleo::transforms::Scaled) -> u32 {
        let len: u32 = (seg.end.x - seg.start.x) + (seg.end.y - seg.start.y);
        return len * s.factor + s.origin;
    }

    @noupgrade
    constructor() {}
}
