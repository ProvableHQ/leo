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

    // Poseidon2 hash tests
    let a1: field = Poseidon2::hash(field_value);
    let b1: field = Poseidon2::hash(i8_value);
    let c1: field = Poseidon2::hash(i16_value);
    let d1: field = Poseidon2::hash(i32_value);
    let e1: field = Poseidon2::hash(i64_value);
    let f1: field = Poseidon2::hash(i128_value);
    let g1: field = Poseidon2::hash(u8_value);
    let h1: field = Poseidon2::hash(u16_value);
    let i1: field = Poseidon2::hash(u32_value);
    let j1: field = Poseidon2::hash(u64_value);
    let k1: field = Poseidon2::hash(u128_value);
    let l1: field = Poseidon2::hash(scalar_value);
    let m1: field = Poseidon2::hash(string_value);

    // Poseidon4 hash tests
    let a2: field = Poseidon4::hash(field_value);
    let b2: field = Poseidon4::hash(i8_value);
    let c2: field = Poseidon4::hash(i16_value);
    let d2: field = Poseidon4::hash(i32_value);
    let e2: field = Poseidon4::hash(i64_value);
    let f2: field = Poseidon4::hash(i128_value);
    let g2: field = Poseidon4::hash(u8_value);
    let h2: field = Poseidon4::hash(u16_value);
    let i2: field = Poseidon4::hash(u32_value);
    let j2: field = Poseidon4::hash(u64_value);
    let k2: field = Poseidon4::hash(u128_value);
    let l2: field = Poseidon4::hash(scalar_value);
    let m2: field = Poseidon4::hash(string_value);

    // Poseidon8 hash tests
    let a3: field = Poseidon8::hash(field_value);
    let b3: field = Poseidon8::hash(i8_value);
    let c3: field = Poseidon8::hash(i16_value);
    let d3: field = Poseidon8::hash(i32_value);
    let e3: field = Poseidon8::hash(i64_value);
    let f3: field = Poseidon8::hash(i128_value);
    let g3: field = Poseidon8::hash(u8_value);
    let h3: field = Poseidon8::hash(u16_value);
    let i3: field = Poseidon8::hash(u32_value);
    let j3: field = Poseidon8::hash(u64_value);
    let k3: field = Poseidon8::hash(u128_value);
    let l3: field = Poseidon8::hash(scalar_value);
    let m3: field = Poseidon8::hash(string_value);

    return true;
}