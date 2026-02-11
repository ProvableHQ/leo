// Test: a library and a submodule share the same name.
// The library should take priority for `utils::VALUE`.

// --- library: utils --- //

const VALUE: u32 = 100u32;
const OTHER: u32 = VALUE + 1u32;

// --- Next Program --- //

// The program has a submodule also named `utils`.
// Since the library takes priority (for now), `utils::VALUE` should resolve to 100 (the library),
// while the submodule's `SHADOW` is inaccessible via `utils::` prefix.

program collision.aleo {
    fn foo() -> u32 {
        // Both of these resolve to the library.
        return utils::VALUE + utils::OTHER;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: utils.leo --- //
// This submodule has the same name as the library.
// It should be shadowed by the library.
const VALUE: u32 = 999u32;
