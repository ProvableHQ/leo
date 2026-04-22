// Test: a library struct that has a field of another library struct, stored in
// main with `.unwrap()` to force zero-value synthesis for the entire nested
// composite.
//
// Exercises recursive `Expression::zero` on a library struct whose member is
// itself a library struct. Both composites must be present in
// `self.composites` (phase-1 collection must traverse both definitions).

// --- library: geo_lib --- //

struct Point {
    x: u32,
    y: u32,
}

struct Rect {
    top_left: Point,
    size_x: u32,
    size_y: u32,
}

// --- Next Program --- //

program canvas.aleo {
    storage selection: geo_lib::Rect;

    fn pick() -> Final {
        return final {
            let r = selection.unwrap();
            assert_eq(r.top_left.x, r.top_left.x);
        };
    }

    @noupgrade
    constructor() {}
}
