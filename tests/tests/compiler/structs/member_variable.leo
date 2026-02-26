
struct Foo {
    x: u32,
}

program test.aleo {
    fn main(y: bool) -> bool {
        let a: Foo = Foo { x: 1u32 };

        return (a.x == 1u32) == y;
    }
}
