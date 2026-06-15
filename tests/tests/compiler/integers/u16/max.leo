program test.aleo {
    fn main(y: bool) -> bool {
        let _a: u16 = 65535u16;
        return y == true;
    }

    @noupgrade
    constructor() {}
}
