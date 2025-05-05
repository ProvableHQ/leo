/*
upgrade = { "mode": "custom" }
*/

program test.aleo {
    
    function foo(a: bool) {
        assert_eq(true, a);
    }
    
    transition bar() {}
    
    async constructor() {
        foo(true);
        bar();
    }
}