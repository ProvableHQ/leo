/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    struct Foo {
        a: u16,
        b: u16,
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
    ) -> u128 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash_to_u128 tests
        let a: u128 = Pedersen64::hash_to_u128(addr_value);
        let b: u128 = Pedersen64::hash_to_u128(bool_value);
        let c: u128 = Pedersen64::hash_to_u128(field_value);
        let d: u128 = Pedersen64::hash_to_u128(group_value);
        let e: u128 = Pedersen64::hash_to_u128(i8_value);
        let f: u128 = Pedersen64::hash_to_u128(i16_value);
        let g: u128 = Pedersen64::hash_to_u128(i32_value);
        let j: u128 = Pedersen64::hash_to_u128(u8_value);
        let k: u128 = Pedersen64::hash_to_u128(u16_value);
        let l: u128 = Pedersen64::hash_to_u128(u32_value);
        let o: u128 = Pedersen64::hash_to_u128(scalar_value);
        // let p: u128 = Pedersen64::hash_to_u128(string_value);
        let q: u128 = Pedersen64::hash_to_u128(Foo { a: 1u16, b: 1u16, });
    
        return a;
    }
}
