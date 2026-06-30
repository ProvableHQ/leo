// FAIL: Only records can be cast to dyn record, not structs.
export struct Foo {
    x: u64,
}

program test.aleo {
    fn main(f: Foo) -> dyn record {
        return f as dyn record;
    }

    @noupgrade
    constructor() {}
}
