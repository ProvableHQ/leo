export const x: u32 = 42;

export struct Foo {
    x: u32,
}

program b28788.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        // Previously the `x` would lead to a panic in codegen.
        let f = Foo { x };
        return f.x;
    }

    @noupgrade
    constructor() {}
}
