program b28610.aleo {
    // This should produce only one type error.
    fn foo() {
        return final { bar(); };
    }
}

final fn bar() {
    assert(true);
}
