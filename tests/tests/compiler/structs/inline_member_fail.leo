
struct Foo {
    x: u8,
}

program test.aleo {
    fn main() {
        let y: u8 = 1;
        let a: Foo = Foo { y };
    }
}
