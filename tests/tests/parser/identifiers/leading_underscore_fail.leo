program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        // single leading underscore, typed
        let _c: u32 = a + b;
        
        // single leading underscore, untyped
        let _temp = 10u32;
        
        // using underscore-prefixed variable in expression
        return _c + _temp;
    }
}

