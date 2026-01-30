@no_inline
fn foo(a: bool) {
    assert_eq(true, a);
}

program test.aleo {
    fn bar() {}

    @custom
    constructor() {
        foo(true);
        bar();
    }
}
