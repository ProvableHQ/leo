program test.aleo {
    fn main() -> u32 {
        return _foo();
    }

    @noupgrade
    constructor() {}
}

@no_inline
fn _foo() -> u32 {
    return 1u32;
}
