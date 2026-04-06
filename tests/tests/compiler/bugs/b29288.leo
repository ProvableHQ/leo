interface Foo {
    fn foo(private a: u8) -> private u8;
}

program scratch.aleo : Foo {
    fn foo(a: u8) -> u8 { return 0; }
}
