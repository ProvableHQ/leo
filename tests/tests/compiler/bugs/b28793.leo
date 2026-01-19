program b28610.aleo {
    // This should produce only one type error.
    async fn foo() {
        return bar();
    }
}

async fn bar() {
    assert(true);
}
