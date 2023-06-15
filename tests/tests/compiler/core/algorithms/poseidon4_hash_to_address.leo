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
    
        // Poseidon4 hash_to_address tests
        let a: address = Poseidon4::hash_to_address(field_value);
        let b: address = Poseidon4::hash_to_address(i8_value);
        let c: address = Poseidon4::hash_to_address(i16_value);
        let d: address = Poseidon4::hash_to_address(i32_value);
        let e: address = Poseidon4::hash_to_address(i64_value);
        let f: address = Poseidon4::hash_to_address(i128_value);
        let g: address = Poseidon4::hash_to_address(u8_value);
        let h: address = Poseidon4::hash_to_address(u16_value);
        let i: address = Poseidon4::hash_to_address(u32_value);
        let j: address = Poseidon4::hash_to_address(u64_value);
        let k: address = Poseidon4::hash_to_address(u128_value);
        let l: address = Poseidon4::hash_to_address(scalar_value);
        // let m: address = Poseidon4::hash_to_address(string_value);
        let n: address = Poseidon4::hash_to_address(Foo { a: 1u128, b: 2u128 });
    
        return field_value;
    }}
