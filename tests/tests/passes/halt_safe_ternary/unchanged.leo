// Test: ternaries with no halting operation in either arm are left completely unchanged,
// and halting operations outside any ternary arm are not predicated.

program test.aleo {
    // No halting operation in either arm: the ternary is untouched.
    fn select(c: bool, a: u8, b: u8) -> u8 {
        return c ? a : b;
    }

    // A comparison in the arm does not halt, so nothing is predicated.
    fn compare(c: bool, a: u8, b: u8) -> bool {
        return c ? a < b : false;
    }

    // A division outside any ternary arm is left unguarded (a genuine fault should halt).
    fn top_level(a: u8, b: u8) -> u8 {
        return a / b;
    }

    @noupgrade
    constructor() {}
}
