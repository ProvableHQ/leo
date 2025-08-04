program test.aleo {
    function foo(a: bool) {
        assert_eq(true, a);
    }
    
    transition bar() {}

    @custom
    async constructor() {
        foo(true);
        bar();
    }
}
