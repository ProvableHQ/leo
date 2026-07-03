// Test: nested ternaries conjoin their conditions, and narrowing casts are predicated.

program test.aleo {
    // The deep `a / b` is guarded by the conjunction of both enclosing conditions.
    fn nested(c1: bool, c2: bool, a: u8, b: u8) -> u8 {
        return c1 ? (c2 ? a / b : 1u8) : 2u8;
    }

    // A narrowing cast's operand is predicated to `0` of the source type.
    fn narrow(c: bool, x: u32) -> u8 {
        return c ? x as u8 : 0u8;
    }

    // A halting op nested inside a larger expression in the arm is still predicated.
    fn compound(c: bool, a: u8, b: u8) -> u8 {
        return c ? 1u8 + a / b : 0u8;
    }

    @noupgrade
    constructor() {}
}
