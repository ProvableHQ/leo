program test.aleo {
    struct Foo {
        value: u32,
    }

    transition main(f: Foo) -> Foo {
        return f;
    }
}
