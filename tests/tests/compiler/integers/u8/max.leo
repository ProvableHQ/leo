program test.aleo {
    fn main(y: bool) -> bool {
        let _a: u8 = 255u8;
        return y == true;
    }

    @noupgrade
    constructor() {}
}
