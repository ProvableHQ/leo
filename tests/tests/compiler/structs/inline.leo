
program test.aleo {
    struct Foo {
        x: u32
    }

    transition main(x: u32) -> u32 {
        let a: Foo = Foo { x: x };
        return a.x;
    }
}
