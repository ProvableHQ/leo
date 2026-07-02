program test.aleo {
    @noupgrade
    constructor() {}

    fn preserve_mul_zero_operand(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        return diff * 0u8;
    }
}
