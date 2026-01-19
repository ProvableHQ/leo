
struct Foo {
    x: u32,
}

program test.aleo {
    fn main(zz: bool) {
        // no member y in Foo
        let a: Foo = Foo { y: 0u32 };
    }
}
