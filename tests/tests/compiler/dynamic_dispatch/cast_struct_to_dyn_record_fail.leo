// FAIL: Only records can be cast to dyn record, not structs.
program test.aleo {
    struct Foo {
        x: u64,
    }

    fn main(f: Foo) -> dyn record {
        return f as dyn record;
    }
}
