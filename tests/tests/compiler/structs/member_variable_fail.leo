
program test.aleo {
    struct Foo {
        x: u32,
    }

    transition main() {
        let a: Foo = Foo { x: 1u32 };
        let err = a.y;
    }
}
