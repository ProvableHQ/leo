program test.aleo {
    // The divisor of a division in a ternary arm is predicated by the arm's condition,
    // so the untaken arm cannot divide by zero.
    fn divide(c: bool, a: u8, b: u8) -> u8 {
        return c ? a / b : 0u8;
    }

    // Signed division also predicates the divisor, covering both division by zero and the
    // signed `MIN / -1` overflow.
    fn signed_divide(c: bool, a: i8, b: i8) -> i8 {
        return c ? a / b : 0i8;
    }

    // Nested ternary arms conjoin their conditions for the deepest halting operation.
    fn nested(c1: bool, c2: bool, a: u8, b: u8) -> u8 {
        return c1 ? (c2 ? a / b : 1u8) : 2u8;
    }

    // A ternary with no halting operation in either arm is unaffected.
    fn select(c: bool, a: u8, b: u8) -> u8 {
        return c ? a : b;
    }

    @noupgrade
    constructor() {}
}
