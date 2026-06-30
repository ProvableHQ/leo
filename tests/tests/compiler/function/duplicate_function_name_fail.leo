program test.aleo {
    fn main() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}

export fn foo() -> u32 {
    return 1u32;
}

export fn foo() -> u32 {
    return 2u32;
}
