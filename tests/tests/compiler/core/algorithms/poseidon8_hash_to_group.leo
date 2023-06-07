/*
namespace: Compile
expectation: Pass
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
    ) -> group {
        let field_value: field = 1field;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Poseidon8 hash_to_group tests
        let a: group = Poseidon8::hash_to_group(field_value);
        let b: group = Poseidon8::hash_to_group(i8_value);
        let c: group = Poseidon8::hash_to_group(i16_value);
        let d: group = Poseidon8::hash_to_group(i32_value);
        let e: group = Poseidon8::hash_to_group(i64_value);
        let f: group = Poseidon8::hash_to_group(i128_value);
        let g: group = Poseidon8::hash_to_group(u8_value);
        let h: group = Poseidon8::hash_to_group(u16_value);
        let i: group = Poseidon8::hash_to_group(u32_value);
        let j: group = Poseidon8::hash_to_group(u64_value);
        let k: group = Poseidon8::hash_to_group(u128_value);
        let l: group = Poseidon8::hash_to_group(scalar_value);
        // let m: group = Poseidon8::hash_to_group(string_value);
        let n: group = Poseidon8::hash_to_group(Foo { a: 1u128, b: 2u128 });
    
        return a + l;
    }}
