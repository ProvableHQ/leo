program operators_basics.aleo {
    fn demo() -> u8 {
        // ANCHOR: arithmetic
        let a: u8 = 1u8 + 1u8;
        // a is equal to 2

        a += 1u8;
        // a is now equal to 3

        a = a.add(1u8);
        // a is now equal to 4
        // ANCHOR_END: arithmetic

        // ANCHOR: parentheses
        let result = (a + 1u8) * 2u8;
        // ANCHOR_END: parentheses

        return result;
    }

    @noupgrade
    constructor() {}
}
