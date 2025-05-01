program test.aleo {
    
    function foo() {
        assert_eq(true, true);
    }
    
    transition bar() {}
    
    async constructor() {
        foo();
        bar();
    }
}