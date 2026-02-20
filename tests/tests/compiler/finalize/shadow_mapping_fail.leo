
program test.aleo {

    mapping bar: u8 => u8;

    fn bar(a: u8) -> u8 {
        return a + 1u8;
    }

    fn foo(bar: u8) -> u8 {
        return bar + 1u8;
    }

    fn baz(a: u8) -> u8 {
        let bar: u8 = a + 1u8;
        return bar;
    }
}

struct bar {
    a: u64,
}
