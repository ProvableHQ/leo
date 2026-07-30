impl Ghost {
    fn make() -> u32 {
        return 1u32;
    }
}

program test.aleo {
    fn main(a: u32) -> u32 {
        return a;
    }

    @noupgrade
    constructor() {}
}
