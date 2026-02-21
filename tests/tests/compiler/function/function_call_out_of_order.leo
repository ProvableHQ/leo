program test.aleo {
    fn foo(a: u8) -> u8 {
        return bar(a) + baz(a);
    }
}

fn bar(a: u8) -> u8 {
    return a * a;
}

fn baz(a: u8) -> u8 {
    return a + a;
}
