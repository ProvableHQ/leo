export struct Foo {
    f: u32,
}

program test.aleo {
    fn main(x: u32) -> Foo {
        let a: u32 = x + 0u32;
        let b: u32 = a * 1u32;
        return Foo { f: b };
    }

    @noupgrade
    constructor() {}
}
