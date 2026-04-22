// Test: composite literal tracking works when the struct type is defined in
// a library. Field accesses on a tracked composite are forwarded to the
// atoms stored for those fields — the fact that the type is imported from a
// library does not affect tracking, because the pass keys on the local
// binding, not on the type's defining module.

// --- library: shape_lib --- //

struct Pair {
    a: u32,
    b: u32,
}

// --- Next Program --- //

program test.aleo {
    fn main(p: u32, q: u32) -> u32 {
        let s: shape_lib::Pair = shape_lib::Pair { a: p, b: q };
        return s.a + s.b;
    }

    @noupgrade
    constructor() {}
}
