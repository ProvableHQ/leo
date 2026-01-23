program b28610.aleo {
    // This should produce only one type error.
    fn foo() {
        return bar();
    }
}

onchain fn bar() {
    assert(true);
}
