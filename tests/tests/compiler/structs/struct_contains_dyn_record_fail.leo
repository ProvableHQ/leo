export struct Wrapper {
    token: dyn record,
}

program test.aleo {
    fn main() -> u8 {
        return 0u8;
    }

    @noupgrade
    constructor() {}
}
