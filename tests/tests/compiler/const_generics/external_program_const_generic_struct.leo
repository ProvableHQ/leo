// Test: const generic structs from a top-level scope and a submodule of an imported .aleo program.

program child.aleo {
    fn make_vec(n: u32) -> types::Vec::[3u32] {
        return types::Vec::[3u32] { len: n };
    }

    fn make_pair(x: u32, y: u32) -> Pair::[3u32] {
        return Pair::[3u32] { x: x, y: y, n: 3u32 };
    }

    @noupgrade
    constructor() {}
}

// Const generic struct at top level of the child program.
struct Pair::[N: u32] {
    x: u32,
    y: u32,
    n: u32,
}

// --- Next Module: types.leo --- //

// Const generic struct in a submodule.
struct Vec::[N: u32] {
    len: u32,
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // Top-level const generic struct as type annotation.
    fn use_pair(p: child.aleo::Pair::[3u32]) -> u32 {
        return p.x + p.y;
    }

    // Top-level const generic struct literal.
    fn make_pair() -> child.aleo::Pair::[3u32] {
        return child.aleo::Pair::[3u32] { x: 1u32, y: 2u32, n: 3u32 };
    }

    // Submodule const generic struct as type annotation.
    fn use_vec(v: child.aleo::types::Vec::[3u32]) -> u32 {
        return v.len;
    }

    // Submodule const generic struct literal.
    fn make_vec() -> child.aleo::types::Vec::[3u32] {
        return child.aleo::types::Vec::[3u32] { len: 3u32 };
    }

    // Call child entry fns that return const generic structs.
    fn use_child_fns(n: u32, x: u32, y: u32) -> u32 {
        let v: child.aleo::types::Vec::[3u32] = child.aleo::make_vec(n);
        let p: child.aleo::Pair::[3u32] = child.aleo::make_pair(x, y);
        return v.len + p.x + p.y;
    }

    @noupgrade
    constructor() {}
}
