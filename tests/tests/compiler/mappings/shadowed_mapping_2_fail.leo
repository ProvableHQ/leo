
program test.aleo {

    mapping one: field => field;

    fn one() -> u8 {
        return 1u8 + 1u8;
    }

    fn foo() -> u8 {
        let one: u8 = 1u8;
        return one + 1u8;
    }
}
