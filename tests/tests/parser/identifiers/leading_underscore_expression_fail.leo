program test.aleo {
    // Test underscore in expression context (not in let statement)
    transition main() -> u32 {
        // This should error when lexer sees _value
        return _value;
    }
    
    // Test in function parameter (should error at lexer level)
    function foo(_param: u32) -> u32 {
        return _param;
    }
}

