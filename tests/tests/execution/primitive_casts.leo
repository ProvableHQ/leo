/*
namespace: Execute
expectation: Pass
cases:
    address_casts:
    - input: ["aleo1hjnn8jdxlzzlqjywamdx4hzcxny5gszzm0q5xmnae4ed6qqn2gys4ugtey"]
    bool_casts:
    - input: ["false"]
    field_casts:
    - input: ["0field"]
    group_casts:
    - input: ["0group"]
    i8_casts:
    - input: ["0i8"]
    i16_casts:
    - input: ["0i16"]
    i32_casts:
    - input: ["0i32"]
    i64_casts:
    - input: ["0i64"]
    i128_casts:
    - input: ["0i128"]
    u8_casts:
    - input: ["0u8"]
    u16_casts:
    - input: ["0u16"]
    u32_casts:
    - input: ["0u32"]
    u64_casts:
    - input: ["0u64"]
    u128_casts:
    - input: ["0u128"]
    scalar_casts:
    - input: ["0scalar"]



*/

program test.aleo {
    transition address_casts(a: address) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (a as address, a as bool, a as field, a as group, a as i8, a as i16, a as i32, a as i64, a as i128, a as u8, a as u16, a as u32, a as u64, a as u128, a as scalar);
    }

    transition bool_casts(b: bool) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (b as address, b as bool, b as field, b as group, b as i8, b as i16, b as i32, b as i64, b as i128, b as u8, b as u16, b as u32, b as u64, b as u128, b as scalar);
    }

    transition field_casts(f: field) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (f as address, f as bool, f as field, f as group, f as i8, f as i16, f as i32, f as i64, f as i128, f as u8, f as u16, f as u32, f as u64, f as u128, f as scalar);
    }

    transition group_casts(g: group) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (g as address, g as bool, g as field, g as group, g as i8, g as i16, g as i32, g as i64, g as i128, g as u8, g as u16, g as u32, g as u64, g as u128, g as scalar);
    }

    transition i8_casts(i: i8) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (i as address, i as bool, i as field, i as group, i as i8, i as i16, i as i32, i as i64, i as i128, i as u8, i as u16, i as u32, i as u64, i as u128, i as scalar);
    }

    transition i16_casts(i: i16) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (i as address, i as bool, i as field, i as group, i as i8, i as i16, i as i32, i as i64, i as i128, i as u8, i as u16, i as u32, i as u64, i as u128, i as scalar);
    }

    transition i32_casts(i: i32) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (i as address, i as bool, i as field, i as group, i as i8, i as i16, i as i32, i as i64, i as i128, i as u8, i as u16, i as u32, i as u64, i as u128, i as scalar);
    }

    transition i64_casts(i: i64) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (i as address, i as bool, i as field, i as group, i as i8, i as i16, i as i32, i as i64, i as i128, i as u8, i as u16, i as u32, i as u64, i as u128, i as scalar);
    }

    transition i128_casts(i: i128) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (i as address, i as bool, i as field, i as group, i as i8, i as i16, i as i32, i as i64, i as i128, i as u8, i as u16, i as u32, i as u64, i as u128, i as scalar);
    }

    transition u8_casts(u: u8) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (u as address, u as bool, u as field, u as group, u as i8, u as i16, u as i32, u as i64, u as i128, u as u8, u as u16, u as u32, u as u64, u as u128, u as scalar);
    }

    transition u16_casts(u: u16) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (u as address, u as bool, u as field, u as group, u as i8, u as i16, u as i32, u as i64, u as i128, u as u8, u as u16, u as u32, u as u64, u as u128, u as scalar);
    }

    transition u32_casts(u: u32) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (u as address, u as bool, u as field, u as group, u as i8, u as i16, u as i32, u as i64, u as i128, u as u8, u as u16, u as u32, u as u64, u as u128, u as scalar);
    }

    transition u64_casts(u: u64) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (u as address, u as bool, u as field, u as group, u as i8, u as i16, u as i32, u as i64, u as i128, u as u8, u as u16, u as u32, u as u64, u as u128, u as scalar);
    }

    transition u128_casts(u: u128) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (u as address, u as bool, u as field, u as group, u as i8, u as i16, u as i32, u as i64, u as i128, u as u8, u as u16, u as u32, u as u64, u as u128, u as scalar);
    }

    transition scalar_casts(s: scalar) -> (address, bool, field, group, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, scalar) {
        return (s as address, s as bool, s as field, s as group, s as i8, s as i16, s as i32, s as i64, s as i128, s as u8, s as u16, s as u32, s as u64, s as u128, s as scalar);
    }
}




