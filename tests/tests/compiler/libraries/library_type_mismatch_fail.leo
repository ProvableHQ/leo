// Test: a library const whose initialiser has the wrong type (boolean assigned to u32).
// Documents whether type errors in library consts are caught at library-parse time
// or deferred until the consuming program is type-checked.

// --- library: bad_lib --- //

const X: u32 = true;

// --- Next Program --- //

program type_mismatch.aleo {
    fn compute() -> u32 {
        return bad_lib::X;
    }

    @noupgrade
    constructor() {}
}
