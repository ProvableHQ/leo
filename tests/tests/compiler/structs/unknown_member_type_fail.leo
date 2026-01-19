
struct Foo {
    a: u8,
    bar: Bar,
}

program test.aleo {
    fn main(a: u8) -> u8 {
        return a + 1u8;
    }
}
