program operators_comprehensive.aleo {
    
    // Test all arithmetic operators
    fn test_all_arithmetic() -> u8 {
        let add: u8? = 1 + 1;
        let sub: u8? = 5 - 3;
        let mul: u8? = 2 * 3;
        let div: u8? = 10 / 2;
        let rem: u8? = 10 % 3;
        let pow: u8? = 2 ** 3u8;
        return 42u8;
    }
    
    // Test all bitwise operators
    fn test_all_bitwise() -> u8 {
        let and: u8? = 5 & 3;
        let or: u8? = 5 | 3;
        let xor: u8? = 5 ^ 3;
        let shl: u8? = 5 << 1u8;
        let shr: u8? = 10 >> 1u8;
        return 42u8;
    }
    
    // Test all wrapped operators
    fn test_all_wrapped() -> u8 {
        let add_w: u8? = 250.add_wrapped(10);
        let sub_w: u8? = 5.sub_wrapped(10);
        let mul_w: u8? = 200.mul_wrapped(2);
        let div_w: u8? = 10.div_wrapped(3);
        let rem_w: u8? = 10.rem_wrapped(3);
        let pow_w: u8? = 2.pow_wrapped(8u8);
        let shl_w: u8? = 128.shl_wrapped(1u8);
        let shr_w: u8? = 1.shr_wrapped(1u8);
        return 42u8;
    }
    
    // Test all unary operators for signed integers
    fn test_all_unary_signed() -> u8 {
        let neg: i8? = -5;
        let abs: i8? = (-10).abs();
        let abs_w: i8? = (-127).abs_wrapped();
        return 42u8;
    }
    
    // Test all unary operators for unsigned integers
    fn test_all_unary_unsigned() -> u8 {
        let not: u8? = !5;
        return 42u8;
    }
    
    // Test all unary operators for bool
    fn test_all_unary_bool() -> u8 {
        let not_bool: bool? = !true;
        return 42u8;
    }
    
    // Test all unary operators for field
    fn test_all_unary_field() -> u8 {
        let neg_f: field? = -1;
        let double_f: field? = (5).double();
        let inv_f: field? = (5).inv();
        let square_f: field? = (5).square();
        let sqrt_f: field? = (25).square_root();
        return 42u8;
    }
    
    // Test complex nested expressions
    fn test_complex_nested() -> u8 {
        let a: u8? = (1 + 2) * (3 + 4);
        let b: u8? = ((5 - 2) * 3) + (10 / 2);
        let c: i8? = -(5 + 3);
        let d: i8? = (-10).abs() + 5;
        return 42u8;
    }
    
    // Test with all integer types
    fn test_all_integer_types() -> u8 {
        let u8_val: u8? = 1 + 1;
        let u16_val: u16? = 100 + 200;
        let u32_val: u32? = 1000 + 2000;
        let u64_val: u64? = 10000 + 20000;
        let u128_val: u128? = 100000 + 200000;
        let i8_val: i8? = -5 + 3;
        let i16_val: i16? = -100 + 200;
        let i32_val: i32? = -1000 + 2000;
        let i64_val: i64? = -10000 + 20000;
        let i128_val: i128? = -100000 + 200000;
        return 42u8;
    }
    
    // Test operator precedence with optionals
    fn test_operator_precedence() -> u8 {
        let a: u8? = 1 + 2 * 3;
        let b: u8? = 10 - 2 / 2;
        let c: u8? = 2 ** 3u8 * 2;
        return 42u8;
    }
    
    // Test comparison operators (return bool, not optional)
    fn test_comparison_operators() -> bool {
        let a: u8 = 5;
        let b: u8 = 3;
        return a > b;
    }
    
    // Test logical operators (return bool, not optional)
    fn test_logical_operators() -> bool {
        return true && false;
    }
}

