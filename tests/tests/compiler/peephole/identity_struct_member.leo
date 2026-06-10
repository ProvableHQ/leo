program test.aleo {
    struct Foo {
        x: u32,
    }

    fn main(s: Foo) -> u32 {
        let y: u32 = s.x + 0u32;
        return y;
    }

    @noupgrade
    constructor() {}
}
