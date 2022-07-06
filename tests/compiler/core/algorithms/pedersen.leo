/*
namespace: Compile
expectation: Pass
*/

function main() -> bool {
    let bool_value: bool = true;
    let i8_value: i8 = 1i8;
    let i16_value: i16 = 1i16;
    let i32_value: i32 = 1i32;
    let i64_value: i64 = 1i64;
    let i128_value: i128 = 1i128;
    let u8_value: u8 = 1u8;
    let u16_value: u16 = 1u16;
    let u32_value: u32 = 1u32;
    let u64_value: u64 = 1u64;
    let u128_value: u128 = 1u128;
    let string_value: string = "helloworld";

    // Pedersen64 commit tests
    let a1: group = Pedersen64::commit(bool_value, 1scalar);
    let b1: group = Pedersen64::commit(i8_value, 1scalar);
    let c1: group = Pedersen64::commit(i16_value, 1scalar);
    let d1: group = Pedersen64::commit(i32_value, 1scalar);
    let e1: group = Pedersen64::commit(i64_value, 1scalar);
    let f1: group = Pedersen64::commit(u8_value, 1scalar);
    let g1: group = Pedersen64::commit(u16_value, 1scalar);
    let h1: group = Pedersen64::commit(u32_value, 1scalar);
    let i1: group = Pedersen64::commit(u64_value, 1scalar);
    let j1: group = Pedersen64::commit(string_value, 1scalar);

    // Pedersen64 hash tests
    let a11: group = Pedersen64::hash(bool_value);
    let b11: group = Pedersen64::hash(i8_value);
    let c11: group = Pedersen64::hash(i16_value);
    let d11: group = Pedersen64::hash(i32_value);
    let e11: group = Pedersen64::hash(i64_value);
    let f11: group = Pedersen64::hash(u8_value);
    let g11: group = Pedersen64::hash(u16_value);
    let h11: group = Pedersen64::hash(u32_value);
    let i11: group = Pedersen64::hash(u64_value);
    let j11: group = Pedersen64::hash(string_value);

    // Pedersen128 commit tests
    let a2: group = Pedersen128::commit(bool_value, 1scalar);
    let b2: group = Pedersen128::commit(i8_value, 1scalar);
    let c2: group = Pedersen128::commit(i16_value, 1scalar);
    let d2: group = Pedersen128::commit(i32_value, 1scalar);
    let e2: group = Pedersen128::commit(i64_value, 1scalar);
    let f2: group = Pedersen128::commit(i128_value, 1scalar);
    let g2: group = Pedersen128::commit(u8_value, 1scalar);
    let h2: group = Pedersen128::commit(u16_value, 1scalar);
    let i2: group = Pedersen128::commit(u32_value, 1scalar);
    let j2: group = Pedersen128::commit(u64_value, 1scalar);
    let k2: group = Pedersen128::commit(u128_value, 1scalar);
    let l2: group = Pedersen128::commit(string_value, 1scalar);

    // Pedersen128 hash tests
    let a22: group = Pedersen128::hash(bool_value);
    let b22: group = Pedersen128::hash(i8_value);
    let c22: group = Pedersen128::hash(i16_value);
    let d22: group = Pedersen128::hash(i32_value);
    let e22: group = Pedersen128::hash(i64_value);
    let f22: group = Pedersen128::hash(i128_value);
    let g22: group = Pedersen128::hash(u8_value);
    let h22: group = Pedersen128::hash(u16_value);
    let i22: group = Pedersen128::hash(u32_value);
    let j22: group = Pedersen128::hash(u64_value);
    let k22: group = Pedersen128::hash(u128_value);
    let l22: group = Pedersen128::hash(string_value);

    return true;
}