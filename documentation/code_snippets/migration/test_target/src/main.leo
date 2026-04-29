program test_program.aleo {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn sum_u8(a: u8, b: u8) -> u8 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}
