// Test: a library submodule defines a struct whose member type is `Optional<primitive>`,
// plus a submodule function that independently uses `Optional<primitive>`.
//
// Verifies that Optional wrapper structs produced while reconstructing a library
// SUBMODULE are attributed to the library (not the submodule) and land in the
// library's top-level `structs` list — not duplicated into the submodule, not
// misrouted to the main program.

// --- library: geo_lib --- //

// --- Next Module: shapes.leo --- //
struct Thing {
    v: u32?,
}

fn make_thing(n: u32) -> Thing {
    let maybe: u32? = none;
    return Thing { v: maybe.unwrap_or(n) };
}

fn value_of(t: Thing) -> u32 {
    return t.v.unwrap_or(0u32);
}

// --- Next Program --- //

program consumer.aleo {
    fn compute(n: u32) -> u32 {
        let t = geo_lib::shapes::make_thing(n);
        return geo_lib::shapes::value_of(t);
    }

    @noupgrade
    constructor() {}
}
