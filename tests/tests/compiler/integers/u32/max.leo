program test.aleo {
    fn main(y: bool) -> bool {
        let _a: u32 = 4294967295u32;
        return y == true;
    }

    @noupgrade
    constructor() {}
}
