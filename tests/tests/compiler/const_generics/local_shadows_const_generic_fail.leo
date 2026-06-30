program test.aleo {
    fn main() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}

export fn foo::[N: u32]() -> u32 {
    let N: u32 = 10u32;
    return N;
}
