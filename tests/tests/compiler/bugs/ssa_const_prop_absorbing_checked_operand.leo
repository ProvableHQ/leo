program test.aleo {
    fn main(x: u8, y: u8) -> u8 {
        let difference: u8 = x - y;
        return difference * 0u8;
    }

    @noupgrade
    constructor() {}
}
