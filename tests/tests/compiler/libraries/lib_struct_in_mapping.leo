// Test: a monomorphized library generic struct used as a mapping value type.
//
// Verifies that CompositeType with const arguments is handled correctly in mapping
// positions (reconstruct_mapping in the monomorphizer has a separate code path from
// function-body struct uses).
//
// Dependency graph:
//   cache_lib  (library: struct Entry::[N: u32])
//   store.aleo (program: mapping store: address => cache_lib::Entry::[8u32])

// --- library: cache_lib --- //

struct Entry::[N: u32] {
    size: u32,
}

// --- Next Program --- //

program store.aleo {
    mapping store: address => cache_lib::Entry::[8u32];

    fn get_size(e: cache_lib::Entry::[8u32]) -> u32 {
        return e.size;
    }

    @noupgrade
    constructor() {}
}
