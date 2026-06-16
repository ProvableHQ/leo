program test.aleo {
    fn main(a: u8) -> u8 {
        let b: u8 = 2u8;
        b *= a;

        return b;
    }

    @noupgrade
    constructor() {}
}
