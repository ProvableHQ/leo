// Test: a struct defined in one library and re-used by a second library, with
// the nested struct stored in the main program and `.unwrap()`ed.
//
// Exercises phase-1 recursion: when the main program imports `shape_lib`, the
// walk must descend into `shape_lib`'s own stubs to register `geo_lib::Point`.
// Without that recursion, `wrap_none` for `Rect` (whose field is `geo_lib::Point`)
// would fail to find members of `Point`.

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

program canvas.aleo {
    storage current: shape_lib::Rect;

    fn measure() -> Final {
        return final {
            let r = current.unwrap();
            assert_eq(r.top_left.x, r.top_left.x);
        };
    }

    @noupgrade
    constructor() {}
}
