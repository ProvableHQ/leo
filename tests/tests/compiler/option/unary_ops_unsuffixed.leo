program unary_ops_unsuffixed.aleo {
    // Test negate with unsuffixed literals
    fn test_negate_unsuffixed() -> u8 {
        let _a: i8? = -5;
        let _b: i16? = -100;
        let _c: i32? = -1000;
        let _d: i64? = -50000;
        let _e: i128? = -1000000;
        let _f: field? = -1;
        return 42u8;
    }

    // Test not with unsuffixed literals
    fn test_not_unsuffixed() -> u8 {
        let _a: bool? = !true;
        let _b: bool? = !false;
        let _c: u8? = !5;
        let _d: u16? = !100;
        return 42u8;
    }

    // Test abs with unsuffixed negative literals
    fn test_abs_unsuffixed() -> u8 {
        let _a: i8? = (-5).abs();
        let _b: i16? = (-100).abs();
        let _c: i32? = (-1000).abs();
        return 42u8;
    }

    // Test abs_wrapped with unsuffixed literals
    fn test_abs_wrapped_unsuffixed() -> u8 {
        let _a: i8? = (-127).abs_wrapped();
        let _b: i16? = (-32767).abs_wrapped();
        return 42u8;
    }

    // Test field operations with unsuffixed literals
    fn test_field_ops_unsuffixed() -> u8 {
        let _a: field? = (5).double();
        let _b: field? = (5).inv();
        let _c: field? = (5).square();
        let _d: field? = (25).square_root();
        return 42u8;
    }

    // Test nested unary operations
    fn test_nested_unary_unsuffixed() -> u8 {
        let _a: i8? = -(-5);
        let _b: bool? = !!true;
        let _c: u8? = !!5;
        return 42u8;
    }

    // Test unary in expression context
    fn test_unary_in_expression() -> u8 {
        let _a: i8? = -5 + 3;
        let _b: i8? = (-10) * 2;
        let _c: bool? = !true && false;
        return 42u8;
    }

    @noupgrade
    constructor() {}
}
