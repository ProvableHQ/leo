program operators_comprehensive.aleo {
    // Test all arithmetic operators
    fn test_all_arithmetic() -> u8 {
        let _add: u8? = 1 + 1;
        let _sub: u8? = 5 - 3;
        let _mul: u8? = 2 * 3;
        let _div: u8? = 10 / 2;
        let _rem: u8? = 10 % 3;
        let _pow: u8? = 2 ** 3u8;
        return 42u8;
    }

    // Test all bitwise operators
    fn test_all_bitwise() -> u8 {
        let _and: u8? = 5 & 3;
        let _or: u8? = 5 | 3;
        let _xor: u8? = 5 ^ 3;
        let _shl: u8? = 5 << 1u8;
        let _shr: u8? = 10 >> 1u8;
        return 42u8;
    }

    // Test all wrapped operators
    fn test_all_wrapped() -> u8 {
        let _add_w: u8? = 250.add_wrapped(10);
        let _sub_w: u8? = 5.sub_wrapped(10);
        let _mul_w: u8? = 200.mul_wrapped(2);
        let _div_w: u8? = 10.div_wrapped(3);
        let _rem_w: u8? = 10.rem_wrapped(3);
        let _pow_w: u8? = 2.pow_wrapped(8u8);
        let _shl_w: u8? = 128.shl_wrapped(1u8);
        let _shr_w: u8? = 1.shr_wrapped(1u8);
        return 42u8;
    }

    // Test all unary operators for signed integers
    fn test_all_unary_signed() -> u8 {
        let _neg: i8? = -5;
        let _abs: i8? = (-10).abs();
        let _abs_w: i8? = (-127).abs_wrapped();
        return 42u8;
    }

    // Test all unary operators for unsigned integers
    fn test_all_unary_unsigned() -> u8 {
        let _not: u8? = !5;
        return 42u8;
    }

    // Test all unary operators for bool
    fn test_all_unary_bool() -> u8 {
        let _not_bool: bool? = !true;
        return 42u8;
    }

    // Test all unary operators for field
    fn test_all_unary_field() -> u8 {
        let _neg_f: field? = -1;
        let _double_f: field? = (5).double();
        let _inv_f: field? = (5).inv();
        let _square_f: field? = (5).square();
        let _sqrt_f: field? = (25).square_root();
        return 42u8;
    }

    // Test complex nested expressions
    fn test_complex_nested() -> u8 {
        let _a: u8? = (1 + 2) * (3 + 4);
        let _b: u8? = ((5 - 2) * 3) + (10 / 2);
        let _c: i8? = -(5 + 3);
        let _d: i8? = (-10).abs() + 5;
        return 42u8;
    }

    // Test with all integer types
    fn test_all_integer_types() -> u8 {
        let _u8_val: u8? = 1 + 1;
        let _u16_val: u16? = 100 + 200;
        let _u32_val: u32? = 1000 + 2000;
        let _u64_val: u64? = 10000 + 20000;
        let _u128_val: u128? = 100000 + 200000;
        let _i8_val: i8? = -5 + 3;
        let _i16_val: i16? = -100 + 200;
        let _i32_val: i32? = -1000 + 2000;
        let _i64_val: i64? = -10000 + 20000;
        let _i128_val: i128? = -100000 + 200000;
        return 42u8;
    }

    // Test operator precedence with optionals
    fn test_operator_precedence() -> u8 {
        let _a: u8? = 1 + 2 * 3;
        let _b: u8? = 10 - 2 / 2;
        let _c: u8? = 2 ** 3u8 * 2;
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

    @noupgrade
    constructor() {}
}
