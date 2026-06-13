program test.aleo {
    fn main(y: bool) -> bool {
        let _a: u64 = 18446744073709551615u64;
        return y == true;
    }

    @noupgrade
    constructor() {}
}
