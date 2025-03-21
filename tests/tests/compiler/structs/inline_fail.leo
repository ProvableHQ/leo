
program test.aleo {
    struct Foo {
        x: u32,
    }

    function main(zz: bool) {
        // no member y in Foo
        let a: Foo = Foo { y: 0u32 };
    }
}
