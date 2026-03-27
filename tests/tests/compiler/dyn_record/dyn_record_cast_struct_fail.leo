program test.aleo {
    struct Foo {
        x: u32,
    }

    fn main(f: Foo) -> dyn {
        // Should fail: cannot cast struct to dyn record
        return f as dyn;
    }
}
