program test.aleo {
    fn main(x: u32) -> u32 {
        return bar::[x]();
    }

    @noupgrade
    constructor() {}
}

export fn bar::[N: u32]() -> u32 {
    return N;
}
