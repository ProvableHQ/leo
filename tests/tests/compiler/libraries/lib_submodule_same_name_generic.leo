// Test: two library submodules define a generic struct with the same name.
// Before the path-preserving fix, both monomorphized versions were stored at
// lib/[Pair::[3u32]], causing the second to silently overwrite the first in
// the symbol table. With the fix they live at lib/[a, Pair::[3u32]] and
// lib/[b, Pair::[3u32]] and do not collide.

// --- library: shape_lib --- //

// --- Next Module: a.leo --- //
struct Pair::[N: u32] {
    x: u32,
}

// --- Next Module: b.leo --- //
struct Pair::[N: u32] {
    y: u32,
}

// --- Next Program --- //

program multi_mod.aleo {
    fn main() -> u32 {
        let p: shape_lib::a::Pair::[3u32] = shape_lib::a::Pair::[3u32] { x: 10u32 };
        let q: shape_lib::b::Pair::[3u32] = shape_lib::b::Pair::[3u32] { y: 20u32 };
        return p.x + q.y;  // 30
    }

    @noupgrade
    constructor() {}
}
