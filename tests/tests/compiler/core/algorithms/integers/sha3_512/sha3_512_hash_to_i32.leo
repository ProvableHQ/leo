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
    ) -> i32 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // SHA3_512 hash_to_i32 tests
        let a: i32 = SHA3_512::hash_to_i32(addr_value);
        let b: i32 = SHA3_512::hash_to_i32(bool_value);
        let c: i32 = SHA3_512::hash_to_i32(field_value);
        let d: i32 = SHA3_512::hash_to_i32(group_value);
        let e: i32 = SHA3_512::hash_to_i32(i8_value);
        let f: i32 = SHA3_512::hash_to_i32(i16_value);
        let g: i32 = SHA3_512::hash_to_i32(i32_value);
        let h: i32 = SHA3_512::hash_to_i32(i64_value);
        let i: i32 = SHA3_512::hash_to_i32(i128_value);
        let j: i32 = SHA3_512::hash_to_i32(u8_value);
        let k: i32 = SHA3_512::hash_to_i32(u16_value);
        let l: i32 = SHA3_512::hash_to_i32(u32_value);
        let m: i32 = SHA3_512::hash_to_i32(u64_value);
        let n: i32 = SHA3_512::hash_to_i32(u128_value);
        let o: i32 = SHA3_512::hash_to_i32(scalar_value);
        // let p: i32 = SHA3_512::hash_to_i32(string_value);
        let q: i32 = SHA3_512::hash_to_i32(Foo { a: 1u128, b: 1u128 });
    
        return a;
    }
}
