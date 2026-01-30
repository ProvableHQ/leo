program b28610.aleo {
    // This should produce only one type error.
    fn foo() {
        return bar();
    }
}

final fn bar() {
    assert(true);
}
