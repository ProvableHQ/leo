
struct Foo {
    x: u8
}

program test.aleo {
    fn main(let x: u8, y: bool) -> bool {
        let a: Foo = Foo { x };

        return (b.x == a.x) == y;
    }
}
