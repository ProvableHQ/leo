/*
namespace: Compile
expectation: Pass
*/

function main() -> bool {
    let owner: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
    let bool_value: bool = true;
    let field_value: field = 1field;
    let group_value: group = (0, 1)group;
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
    let scalar_value: scalar = 1scalar;
    let string_value: string = "helloworld";

    // BHP256 commit tests
    let a1: field = BHP256::commit(owner, 1scalar);
    let b1: field = BHP256::commit(bool_value, 1scalar);
    let c1: field = BHP256::commit(field_value, 1scalar);
    let d1: field = BHP256::commit(group_value, 1scalar);
    let e1: field = BHP256::commit(i8_value, 1scalar);
    let f1: field = BHP256::commit(i16_value, 1scalar);
    let g1: field = BHP256::commit(i32_value, 1scalar);
    let h1: field = BHP256::commit(i64_value, 1scalar);
    let i1: field = BHP256::commit(i128_value, 1scalar);
    let j1: field = BHP256::commit(u8_value, 1scalar);
    let k1: field = BHP256::commit(u16_value, 1scalar);
    let l1: field = BHP256::commit(u32_value, 1scalar);
    let m1: field = BHP256::commit(u64_value, 1scalar);
    let n1: field = BHP256::commit(u128_value, 1scalar);
    let o1: field = BHP256::commit(scalar_value, 1scalar);
    let p1: field = BHP256::commit(string_value, 1scalar);

    // BHP256 hash tests
    let a11: field = BHP256::hash(owner);
    let b11: field = BHP256::hash(bool_value);
    let c11: field = BHP256::hash(field_value);
    let d11: field = BHP256::hash(group_value);
    let e11: field = BHP256::hash(i8_value);
    let f11: field = BHP256::hash(i16_value);
    let g11: field = BHP256::hash(i32_value);
    let h11: field = BHP256::hash(i64_value);
    let i11: field = BHP256::hash(i128_value);
    let j11: field = BHP256::hash(u8_value);
    let k11: field = BHP256::hash(u16_value);
    let l11: field = BHP256::hash(u32_value);
    let m11: field = BHP256::hash(u64_value);
    let n11: field = BHP256::hash(u128_value);
    let o11: field = BHP256::hash(scalar_value);
    let p11: field = BHP256::hash(string_value);

    // BHP512 commit tests
    let a2: field = BHP512::commit(owner, 1scalar);
    let b2: field = BHP512::commit(bool_value, 1scalar);
    let c2: field = BHP512::commit(field_value, 1scalar);
    let d2: field = BHP512::commit(group_value, 1scalar);
    let e2: field = BHP512::commit(i8_value, 1scalar);
    let f2: field = BHP512::commit(i16_value, 1scalar);
    let g2: field = BHP512::commit(i32_value, 1scalar);
    let h2: field = BHP512::commit(i64_value, 1scalar);
    let i2: field = BHP512::commit(i128_value, 1scalar);
    let j2: field = BHP512::commit(u8_value, 1scalar);
    let k2: field = BHP512::commit(u16_value, 1scalar);
    let l2: field = BHP512::commit(u32_value, 1scalar);
    let m2: field = BHP512::commit(u64_value, 1scalar);
    let n2: field = BHP512::commit(u128_value, 1scalar);
    let o2: field = BHP512::commit(scalar_value, 1scalar);
    let p2: field = BHP512::commit(string_value, 1scalar);

    // BHP512 hash tests
    let a22: field = BHP512::hash(owner);
    let b22: field = BHP512::hash(bool_value);
    let c22: field = BHP512::hash(field_value);
    let d22: field = BHP512::hash(group_value);
    let e22: field = BHP512::hash(i8_value);
    let f22: field = BHP512::hash(i16_value);
    let g22: field = BHP512::hash(i32_value);
    let h22: field = BHP512::hash(i64_value);
    let i22: field = BHP512::hash(i128_value);
    let j22: field = BHP512::hash(u8_value);
    let k22: field = BHP512::hash(u16_value);
    let l22: field = BHP512::hash(u32_value);
    let m22: field = BHP512::hash(u64_value);
    let n22: field = BHP512::hash(u128_value);
    let o22: field = BHP512::hash(scalar_value);
    let p22: field = BHP512::hash(string_value);

    // BHP768 commit tests
    let a3: field = BHP768::commit(owner, 1scalar);
    let b3: field = BHP768::commit(bool_value, 1scalar);
    let c3: field = BHP768::commit(field_value, 1scalar);
    let d3: field = BHP768::commit(group_value, 1scalar);
    let e3: field = BHP768::commit(i8_value, 1scalar);
    let f3: field = BHP768::commit(i16_value, 1scalar);
    let g3: field = BHP768::commit(i32_value, 1scalar);
    let h3: field = BHP768::commit(i64_value, 1scalar);
    let i3: field = BHP768::commit(i128_value, 1scalar);
    let j3: field = BHP768::commit(u8_value, 1scalar);
    let k3: field = BHP768::commit(u16_value, 1scalar);
    let l3: field = BHP768::commit(u32_value, 1scalar);
    let m3: field = BHP768::commit(u64_value, 1scalar);
    let n3: field = BHP768::commit(u128_value, 1scalar);
    let o3: field = BHP768::commit(scalar_value, 1scalar);
    let p3: field = BHP768::commit(string_value, 1scalar);

    // BHP768 hash tests
    let a33: field = BHP768::hash(owner);
    let b33: field = BHP768::hash(bool_value);
    let c33: field = BHP768::hash(field_value);
    let d33: field = BHP768::hash(group_value);
    let e33: field = BHP768::hash(i8_value);
    let f33: field = BHP768::hash(i16_value);
    let g33: field = BHP768::hash(i32_value);
    let h33: field = BHP768::hash(i64_value);
    let i33: field = BHP768::hash(i128_value);
    let j33: field = BHP768::hash(u8_value);
    let k33: field = BHP768::hash(u16_value);
    let l33: field = BHP768::hash(u32_value);
    let m33: field = BHP768::hash(u64_value);
    let n33: field = BHP768::hash(u128_value);
    let o33: field = BHP768::hash(scalar_value);
    let p33: field = BHP768::hash(string_value);

    // BHP1024 commit tests
    let a4: field = BHP1024::commit(owner, 1scalar);
    let b4: field = BHP1024::commit(bool_value, 1scalar);
    let c4: field = BHP1024::commit(field_value, 1scalar);
    let d4: field = BHP1024::commit(group_value, 1scalar);
    let e4: field = BHP1024::commit(i8_value, 1scalar);
    let f4: field = BHP1024::commit(i16_value, 1scalar);
    let g4: field = BHP1024::commit(i32_value, 1scalar);
    let h4: field = BHP1024::commit(i64_value, 1scalar);
    let i4: field = BHP1024::commit(i128_value, 1scalar);
    let j4: field = BHP1024::commit(u8_value, 1scalar);
    let k4: field = BHP1024::commit(u16_value, 1scalar);
    let l4: field = BHP1024::commit(u32_value, 1scalar);
    let m4: field = BHP1024::commit(u64_value, 1scalar);
    let n4: field = BHP1024::commit(u128_value, 1scalar);
    let o4: field = BHP1024::commit(scalar_value, 1scalar);
    let p4: field = BHP1024::commit(string_value, 1scalar);

    // BHP1024 hash tests
    let a44: field = BHP1024::hash(owner);
    let b44: field = BHP1024::hash(bool_value);
    let c44: field = BHP1024::hash(field_value);
    let d44: field = BHP1024::hash(group_value);
    let e44: field = BHP1024::hash(i8_value);
    let f44: field = BHP1024::hash(i16_value);
    let g44: field = BHP1024::hash(i32_value);
    let h44: field = BHP1024::hash(i64_value);
    let i44: field = BHP1024::hash(i128_value);
    let j44: field = BHP1024::hash(u8_value);
    let k44: field = BHP1024::hash(u16_value);
    let l44: field = BHP1024::hash(u32_value);
    let m44: field = BHP1024::hash(u64_value);
    let n44: field = BHP1024::hash(u128_value);
    let o44: field = BHP1024::hash(scalar_value);
    let p44: field = BHP1024::hash(string_value);

    return true;
}

