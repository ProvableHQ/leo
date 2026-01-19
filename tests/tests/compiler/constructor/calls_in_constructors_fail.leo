fn foo(a: bool) {
    assert_eq(true, a);
}

program test.aleo {
    fn bar() {}

    @custom
    async constructor() {
        foo(true);
        bar();
    }
}
