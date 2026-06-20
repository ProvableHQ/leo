program test.aleo {
    fn main() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}

export fn foo::[N: u32](N: u32) -> u32 {
    return N;
}
