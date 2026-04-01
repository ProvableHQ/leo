// Test: interface with array mapping key in a library submodule does not panic.
// The type checker must visit interface mappings inside library submodules so that
// the type table is populated for the array length literal. Without the fix to
// visit_module, SSA const propagation panics when reconstructing the interface.

// --- library: token_lib --- //

// --- Next Module: token_types.leo --- //

interface TokenStorage {
    mapping allowances: [address; 2] => u128;
}

// --- Next Program --- //

program test.aleo {
    fn foo() {}

    @noupgrade
    constructor() {}
}
