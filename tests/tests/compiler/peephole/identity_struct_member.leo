struct Foo {
    x: u32,
}

program test.aleo {
    fn main(s: Foo) -> u32 {
        let y: u32 = s.x + 0u32;
        return y;
    }

    @noupgrade
    constructor() {}
}
