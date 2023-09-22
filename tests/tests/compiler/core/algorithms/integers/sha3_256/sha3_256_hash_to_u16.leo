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
    ) -> u16 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // SHA3_256 hash_to_u16 tests
        let a: u16 = SHA3_256::hash_to_u16(addr_value);
        let b: u16 = SHA3_256::hash_to_u16(bool_value);
        let c: u16 = SHA3_256::hash_to_u16(field_value);
        let d: u16 = SHA3_256::hash_to_u16(group_value);
        let e: u16 = SHA3_256::hash_to_u16(i8_value);
        let f: u16 = SHA3_256::hash_to_u16(i16_value);
        let g: u16 = SHA3_256::hash_to_u16(i32_value);
        let h: u16 = SHA3_256::hash_to_u16(i64_value);
        let i: u16 = SHA3_256::hash_to_u16(i128_value);
        let j: u16 = SHA3_256::hash_to_u16(u8_value);
        let k: u16 = SHA3_256::hash_to_u16(u16_value);
        let l: u16 = SHA3_256::hash_to_u16(u32_value);
        let m: u16 = SHA3_256::hash_to_u16(u64_value);
        let n: u16 = SHA3_256::hash_to_u16(u128_value);
        let o: u16 = SHA3_256::hash_to_u16(scalar_value);
        // let p: u16 = SHA3_256::hash_to_u16(string_value);
        let q: u16 = SHA3_256::hash_to_u16(Foo { a: 1u128, b: 1u128 });
    
        return a;
    }
}
