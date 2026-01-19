
struct Foo {
    x: u32
}

program test.aleo {
    fn main(x: u32) -> u32 {
        let a: Foo = Foo { x: x };
        return a.x;
    }
}
