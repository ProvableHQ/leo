program style_branches.aleo {
    fn if_else_demo(condition: bool, a: u8, b: u8) -> u8 {
        // ANCHOR: if_else
        if (condition) {
            return a;
        } else {
            return b;
        }
        // ANCHOR_END: if_else
    }

    fn ternary_demo(condition: bool, a: u8, b: u8) -> u8 {
        // ANCHOR: ternary
        return condition ? a : b;
        // ANCHOR_END: ternary
    }

    fn branches(condition: bool, a: u8, b: u8) -> u8 {
        if condition {
            // ANCHOR: branch_a
            return a;
            // ANCHOR_END: branch_a
        }
        // ANCHOR: branch_b
        return b;
        // ANCHOR_END: branch_b
    }

    @noupgrade
    constructor() {}
}
