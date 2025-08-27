program st.aleo {
    struct Foo {
        x: u32?
    }

    mapping optional_balances: address? => Foo;

    transition foo() {
    }
}
