program test.aleo {
    fn main(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        return diff * 0u8;
    }

    @noupgrade
    constructor() {}
}
