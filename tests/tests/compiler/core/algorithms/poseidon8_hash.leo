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
    ) -> field {
        let field_value: field = 1field;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Poseidon8 hash tests
        let a: field = Poseidon8::hash(field_value);
        let b: field = Poseidon8::hash(i8_value);
        let c: field = Poseidon8::hash(i16_value);
        let d: field = Poseidon8::hash(i32_value);
        let e: field = Poseidon8::hash(i64_value);
        let f: field = Poseidon8::hash(i128_value);
        let g: field = Poseidon8::hash(u8_value);
        let h: field = Poseidon8::hash(u16_value);
        let i: field = Poseidon8::hash(u32_value);
        let j: field = Poseidon8::hash(u64_value);
        let k: field = Poseidon8::hash(u128_value);
        let l: field = Poseidon8::hash(scalar_value);
        // let m: field = Poseidon8::hash(string_value);
        let n: field = Poseidon2::hash(Foo { a: 1u128, b: 2u128 });
    
        return a + l;
    }}
