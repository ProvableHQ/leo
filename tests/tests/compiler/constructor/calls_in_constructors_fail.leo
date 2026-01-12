function foo(a: bool) {
    assert_eq(true, a);
}

program test.aleo {
    transition bar() {}

    @custom
    async constructor() {
        foo(true);
        bar();
    }
}
