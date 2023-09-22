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
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Keccak384 hash_to_field tests
        let a: field = Keccak384::hash_to_field(addr_value);
        let b: field = Keccak384::hash_to_field(bool_value);
        let c: field = Keccak384::hash_to_field(field_value);
        let d: field = Keccak384::hash_to_field(group_value);
        let e: field = Keccak384::hash_to_field(i8_value);
        let f: field = Keccak384::hash_to_field(i16_value);
        let g: field = Keccak384::hash_to_field(i32_value);
        let h: field = Keccak384::hash_to_field(i64_value);
        let i: field = Keccak384::hash_to_field(i128_value);
        let j: field = Keccak384::hash_to_field(u8_value);
        let k: field = Keccak384::hash_to_field(u16_value);
        let l: field = Keccak384::hash_to_field(u32_value);
        let m: field = Keccak384::hash_to_field(u64_value);
        let n: field = Keccak384::hash_to_field(u128_value);
        let o: field = Keccak384::hash_to_field(scalar_value);
        // let p: field = Keccak384::hash_to_field(string_value);
        let q: field = Keccak384::hash_to_field(Foo { a: 1u128, b: 1u128 });
    
        return a + o;
    }}
