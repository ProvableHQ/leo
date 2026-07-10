program test.aleo {
    fn main(x: u32) -> u32 {
        assert(true);
        assert(true);
        assert(true);
        return identity(x);
    }

    @noupgrade
    constructor() {}
}

export fn identity(x: u32) -> u32 {
    return x;
}
