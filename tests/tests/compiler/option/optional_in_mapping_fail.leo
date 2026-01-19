struct Foo {
    x: u32?
}

program st.aleo {
    mapping optional_balances: address? => Foo;

    fn foo() {
    }
}
