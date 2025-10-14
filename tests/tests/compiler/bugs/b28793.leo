program b28610.aleo {
    // This should produce only one type error.
    async transition foo() {
        return bar();
    }

    async function bar() {
        assert(true);
    }
}
