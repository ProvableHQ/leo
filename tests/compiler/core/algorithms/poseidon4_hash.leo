/*
namespace: Compile
expectation: Pass
input_file: inputs/int64.in
*/

program test.aleo {    
    struct Foo {
        a: u128,
        b: u128,
    }
    
    transition main(
        i8_value: i8,
        i16_value: i16,
        i32_value: i32,
        i64_value: i64,
        u8_value: u8,
        u16_value: u16,
        u32_value: u32,
        u64_value: u64,
    ) -> field {
        let field_value: field = 1field;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Poseidon4 hash tests
        let a: field = Poseidon4::hash(field_value);
        let b: field = Poseidon4::hash(i8_value);
        let c: field = Poseidon4::hash(i16_value);
        let d: field = Poseidon4::hash(i32_value);
        let e: field = Poseidon4::hash(i64_value);
        let f: field = Poseidon4::hash(i128_value);
        let g: field = Poseidon4::hash(u8_value);
        let h: field = Poseidon4::hash(u16_value);
        let i: field = Poseidon4::hash(u32_value);
        let j: field = Poseidon4::hash(u64_value);
        let k: field = Poseidon4::hash(u128_value);
        let l: field = Poseidon4::hash(scalar_value);
        // let m: field = Poseidon4::hash(string_value);
        let n: field = Poseidon2::hash(Foo { a: 1u128, b: 2u128 });
    
        return a + l;
    }}
