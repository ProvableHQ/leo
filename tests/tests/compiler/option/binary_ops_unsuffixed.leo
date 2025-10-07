program binary_ops_unsuffixed.aleo {
    
    // Test arithmetic operators with unsuffixed literals
    transition test_arithmetic_unsuffixed() -> u8 {
        let a: u8? = 1 + 1;
        let b: u8? = 10 - 5;
        let c: u8? = 2 * 3;
        let d: u8? = 10 / 2;
        let e: u8? = 10 % 3;
        let f: u8? = 2 ** 3u8;
        return 42u8;
    }
    
    // Test bitwise operators with unsuffixed literals
    transition test_bitwise_unsuffixed() -> u8 {
        let a: u8? = 5 & 3;
        let b: u8? = 5 | 3;
        let c: u8? = 5 ^ 3;
        let d: u8? = 5 << 1u8;
        let e: u8? = 10 >> 1u8;
        return 42u8;
    }
    
    // Test wrapped operators with unsuffixed literals
    transition test_wrapped_unsuffixed() -> u8 {
        let a: u8? = 250.add_wrapped(10);
        let b: u8? = 5.sub_wrapped(10);
        let c: u8? = 200.mul_wrapped(2);
        let d: u8? = 10.div_wrapped(3);
        return 42u8;
    }
    
    // Test with different integer types
    transition test_different_int_types() -> u8 {
        let a: u16? = 100 + 200;
        let b: u32? = 1000 * 2000;
        let c: u64? = 5000 - 2000;
        let d: u128? = 100000 / 2;
        return 42u8;
    }
    
    // Test with field type
    transition test_field_unsuffixed() -> u8 {
        let a: field? = 1 + 2;
        let b: field? = 10 * 5;
        let c: field? = 100 - 50;
        let d: field? = 100 / 10;
        return 42u8;
    }
    
    // Test mixed: one suffixed, one unsuffixed
    transition test_mixed_suffixed_unsuffixed() -> u8 {
        let a: u8? = 1u8 + 1;
        let b: u8? = 10 - 5u8;
        let c: u8? = 2u8 * 3u8;
        return 42u8;
    }
    
    // Test with negative unsuffixed literals (signed types)
    transition test_signed_unsuffixed() -> u8 {
        let a: i8? = -5 + 3;
        let b: i16? = 100 - 200;
        let c: i32? = -10 * 5;
        let d: i64? = 100 / -5;
        return 42u8;
    }
    
    // Test nested expressions
    transition test_nested_unsuffixed() -> u8 {
        let a: u8? = (1 + 2) * 3;
        let b: u8? = 10 / (2 + 3);
        let c: u8? = (5 - 2) + (10 - 7);
        return 42u8;
    }
}

