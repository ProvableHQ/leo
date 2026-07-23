program test.aleo {
    fn main() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}

export const N: u32 = 5;

export fn foo::[N: u32]() {}
