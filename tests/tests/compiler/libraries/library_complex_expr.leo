// Test: library consts defined with complex expressions (grouping, chained arithmetic).
// Verifies that expression handling in library consts works the same as in programs.

// --- library: calc_lib --- //

const A: u32 = 3u32;
const B: u32 = 4u32;
const C: u32 = 5u32;
const RESULT: u32 = (A + B) * C;

// --- Next Program --- //

program complex_expr.aleo {
    // A=3, B=4, C=5 → RESULT=(3+4)*5=35
    fn compute() -> u32 {
        return calc_lib::RESULT;
    }

    @noupgrade
    constructor() {}
}
