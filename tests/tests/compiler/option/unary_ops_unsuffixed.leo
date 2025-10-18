program unary_ops_unsuffixed.aleo {
    
    // Test negate with unsuffixed literals
    transition test_negate_unsuffixed() -> u8 {
        let a: i8? = -5;
        let b: i16? = -100;
        let c: i32? = -1000;
        let d: i64? = -50000;
        let e: i128? = -1000000;
        let f: field? = -1;
        return 42u8;
    }
    
    // Test not with unsuffixed literals
    transition test_not_unsuffixed() -> u8 {
        let a: bool? = !true;
        let b: bool? = !false;
        let c: u8? = !5;
        let d: u16? = !100;
        return 42u8;
    }
    
    // Test abs with unsuffixed negative literals
    transition test_abs_unsuffixed() -> u8 {
        let a: i8? = (-5).abs();
        let b: i16? = (-100).abs();
        let c: i32? = (-1000).abs();
        return 42u8;
    }
    
    // Test abs_wrapped with unsuffixed literals
    transition test_abs_wrapped_unsuffixed() -> u8 {
        let a: i8? = (-127).abs_wrapped();
        let b: i16? = (-32767).abs_wrapped();
        return 42u8;
    }
    
    // Test field operations with unsuffixed literals
    transition test_field_ops_unsuffixed() -> u8 {
        let a: field? = (5).double();
        let b: field? = (5).inv();
        let c: field? = (5).square();
        let d: field? = (25).square_root();
        return 42u8;
    }
    
    // Test nested unary operations
    transition test_nested_unary_unsuffixed() -> u8 {
        let a: i8? = -(-5);
        let b: bool? = !!true;
        let c: u8? = !!5;
        return 42u8;
    }
    
    // Test unary in expression context
    transition test_unary_in_expression() -> u8 {
        let a: i8? = -5 + 3;
        let b: i8? = (-10) * 2;
        let c: bool? = !true && false;
        return 42u8;
    }
}

