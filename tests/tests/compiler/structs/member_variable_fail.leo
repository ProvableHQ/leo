
struct Foo {
    x: u32,
}

program test.aleo {
    fn main() {
        let a: Foo = Foo { x: 1u32 };
        let err = a.y;
    }
}
