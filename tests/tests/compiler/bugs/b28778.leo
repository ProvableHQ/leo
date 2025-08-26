program b28788.aleo {
    const x: u32 = 42;

    struct Foo {
        x: u32,
    }

    transition main(public a: u32, b: u32) -> u32 {
        // Previously the `x` would lead to a panic in codegen.
        let f = Foo { x };
        return f.x;
    }
}
