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
    ) -> scalar {
        let field_value: field = 1field;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Poseidon8 hash_to_scalar tests
        let a: scalar = Poseidon8::hash_to_scalar(field_value);
        let b: scalar = Poseidon8::hash_to_scalar(i8_value);
        let c: scalar = Poseidon8::hash_to_scalar(i16_value);
        let d: scalar = Poseidon8::hash_to_scalar(i32_value);
        let e: scalar = Poseidon8::hash_to_scalar(i64_value);
        let f: scalar = Poseidon8::hash_to_scalar(i128_value);
        let g: scalar = Poseidon8::hash_to_scalar(u8_value);
        let h: scalar = Poseidon8::hash_to_scalar(u16_value);
        let i: scalar = Poseidon8::hash_to_scalar(u32_value);
        let j: scalar = Poseidon8::hash_to_scalar(u64_value);
        let k: scalar = Poseidon8::hash_to_scalar(u128_value);
        let l: scalar = Poseidon8::hash_to_scalar(scalar_value);
        // let m: scalar = Poseidon8::hash_to_scalar(string_value);
        let n: scalar = Poseidon8::hash_to_scalar(Foo { a: 1u128, b: 2u128 });
    
        return a + l;
    }}
