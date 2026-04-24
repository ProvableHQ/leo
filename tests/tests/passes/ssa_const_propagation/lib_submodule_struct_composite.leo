// Test: composite literal tracking works for a struct defined in a library
// submodule. Same as `lib_struct_composite_forwarding.leo`, but the struct
// lives one level deeper (in `shape_lib::shapes`) to exercise submodule
// type paths.

// --- library: shape_lib --- //

// --- Next Module: shapes.leo --- //

struct Pair {
    a: u32,
    b: u32,
}

// --- Next Program --- //

program test.aleo {
    fn main(p: u32, q: u32) -> u32 {
        let s: shape_lib::shapes::Pair = shape_lib::shapes::Pair { a: p, b: q };
        return s.a + s.b;
    }

    @noupgrade
    constructor() {}
}
