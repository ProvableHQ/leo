// Test: a 3-level deep library chain (leaf -> middle -> top) where each level
// defines a struct whose member is the struct from the level below, and the
// top-level struct is stored in the main program with `.unwrap()` forcing
// recursive zero-value synthesis.
//
// Exercises phase-1 DFS depth: the walk must descend through multiple
// `Stub::FromLibrary` hops (main -> top_lib -> middle_lib -> leaf_lib) and
// register every composite along the way. Without that recursion, the zero
// value for `Capsule` could not be constructed because the leaf member
// `leaf_lib::Atom` would be missing from `self.composites`.

// --- library: leaf_lib --- //

struct Atom {
    n: u32,
}

// --- Next Program --- //

// --- library: middle_lib --- //

struct Core {
    a: leaf_lib::Atom,
    tag: u32,
}

// --- Next Program --- //

// --- library: top_lib --- //

struct Capsule {
    c: middle_lib::Core,
    extra: u32,
}

// --- Next Program --- //

program factory.aleo {
    storage latest: top_lib::Capsule;

    fn make() -> Final {
        return final {
            let cap = latest.unwrap();
            assert_eq(cap.c.a.n, cap.c.a.n);
        };
    }

    @noupgrade
    constructor() {}
}
