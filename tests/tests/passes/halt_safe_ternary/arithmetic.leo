// Test: each halting integer operation in a ternary arm is predicated by the arm's condition.

program test.aleo {
    // Division: the divisor is predicated to `1`.
    fn divide(c: bool, a: u8, b: u8) -> u8 {
        return c ? a / b : 0u8;
    }

    // Remainder: the divisor is predicated to `1`.
    fn remainder(c: bool, a: u8, b: u8) -> u8 {
        return c ? a % b : 0u8;
    }

    // Addition: both operands are predicated to `0`.
    fn add(c: bool, a: u8, b: u8) -> u8 {
        return c ? a + b : 0u8;
    }

    // Subtraction: both operands are predicated to `0`.
    fn subtract(c: bool, a: u8, b: u8) -> u8 {
        return c ? a - b : 0u8;
    }

    // Multiplication: both operands are predicated to `0`.
    fn multiply(c: bool, a: u8, b: u8) -> u8 {
        return c ? a * b : 0u8;
    }

    // Exponentiation: the exponent is predicated to `0`.
    fn power(c: bool, a: u8, b: u8) -> u8 {
        return c ? a ** b : 0u8;
    }

    // Shift: the shift amount is predicated to `0`.
    fn shift_left(c: bool, a: u8, s: u8) -> u8 {
        return c ? a << s : 0u8;
    }

    // The false arm is predicated by the negation of the condition.
    fn false_arm(c: bool, a: u8, b: u8) -> u8 {
        return c ? 0u8 : a / b;
    }

    @noupgrade
    constructor() {}
}
