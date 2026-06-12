struct Foo {
    x: u32,
}

program test.aleo {
    fn main(f: Foo) -> dyn record {
        // Should fail: cannot cast struct to dyn record
        return f as dyn record;
    }

    @noupgrade
    constructor() {}
}
