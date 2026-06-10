program test.aleo {
    fn main(x: u32) -> u32 {
        assert(true);
        assert_eq(x, x);
        return x;
    }

    @noupgrade
    constructor() {}
}
