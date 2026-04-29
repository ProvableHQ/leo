// ANCHOR: file
struct Foo {
    x: u8
}

program type_inference_demo.aleo {
    fn build() -> Foo {
        let f = Foo {
            x: 5, // inferred to be a `u8`
        };
        return f;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
