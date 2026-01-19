program test.aleo {
    // Tests underscore in expression context
    fn main() -> u32 {
        // this should error when lexer sees _value
        return _value;
    }
}

fn foo(_param: u32) -> u32 {
    return _param;
}

