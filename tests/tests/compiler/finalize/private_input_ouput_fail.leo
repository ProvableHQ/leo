
program test.aleo {
    fn foo(public a: u8) -> (u8, Final) {
        async finalize_foo(a);
        return a + a;
    }
    fn bar(public a: u8) -> (u8, Final) {
        async finalize_bar(a);
        return a + a;
    }
}

onchain fn finalize_foo(private a: u8) -> u8 {
    return a * a;
}

onchain fn finalize_bar(a: u8) -> private u8 {
    return a * a;
}
