
struct Foo {
    constant a: u8,
    private bar: bool,
    public bax: u16,
    baz: u32,
}

program test.aleo {
    fn main(a: u8) -> u8 {
        return a + 1u8;
    }
}
