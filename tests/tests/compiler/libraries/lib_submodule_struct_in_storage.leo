// Test: a struct defined in a library submodule used as a storage type in main,
// with `.unwrap()` forcing the option-lowering pass to synthesize a zero value
// for the submodule struct.
//
// Exercises phase-1 `collect_composite` for library submodules: the composite
// must be registered under `Location(library, [submodule, Struct])` so that
// `wrap_none` can look up its members during zero-synthesis.

// --- library: geo_lib --- //

// --- Next Module: shapes.leo --- //
struct Rect {
    width: u32,
    height: u32,
}

// --- Next Program --- //

program rects.aleo {
    storage bbox: geo_lib::shapes::Rect;

    fn area() -> Final {
        return final {
            let r = bbox.unwrap();
            assert_eq(r.width * r.height, r.width * r.height);
        };
    }

    @noupgrade
    constructor() {}
}
