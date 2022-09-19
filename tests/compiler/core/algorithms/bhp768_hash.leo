/*
namespace: Compile
expectation: Pass
input_file: inputs/int64.in
*/

circuit Foo {
    a: u128,
    b: u128,
}

@program
function main(
    i8_value: i8,
    i16_value: i16,
    i32_value: i32,
    i64_value: i64,
    u8_value: u8,
    u16_value: u16,
    u32_value: u32,
    u64_value: u64,
) -> field {
    let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
    let bool_value: bool = true;
    let field_value: field = 1field;
    // let group_value: group = (0, 1)group;
    let i128_value: i128 = 1i128;
    let u128_value: u128 = 1u128;
    let scalar_value: scalar = 1scalar;
    // let string_value: string = "helloworld";

    // BHP768 hash tests
    let a: field = BHP768::hash(addr_value);
    let b: field = BHP768::hash(bool_value);
    let c: field = BHP768::hash(field_value);
    // let d: field = BHP768::hash(group_value);
    let e: field = BHP768::hash(i8_value);
    let f: field = BHP768::hash(i16_value);
    let g: field = BHP768::hash(i32_value);
    let h: field = BHP768::hash(i64_value);
    let i: field = BHP768::hash(i128_value);
    let j: field = BHP768::hash(u8_value);
    let k: field = BHP768::hash(u16_value);
    let l: field = BHP768::hash(u32_value);
    let m: field = BHP768::hash(u64_value);
    let n: field = BHP768::hash(u128_value);
    let o: field = BHP768::hash(scalar_value);
    // let p: field = BHP768::hash(string_value);
    let q: field = BHP256::hash(Foo { a: 1u128, b: 1u128 });

    return a + o;
}