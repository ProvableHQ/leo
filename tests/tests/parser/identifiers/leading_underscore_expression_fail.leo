program test.aleo {
    // Tests underscore in expression context
    transition main() -> u32 {
        // this should error when lexer sees _value
        return _value;
    }
    
    function foo(_param: u32) -> u32 {
        return _param;
    }
}

