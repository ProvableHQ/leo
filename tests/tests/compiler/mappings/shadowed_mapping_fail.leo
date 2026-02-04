
program test.aleo {

    mapping one: field => field;
    mapping one: field => field;

    transition foo() -> u8 {
        let one: u8 = 1u8;
        return one + 1u8;
    }
}
