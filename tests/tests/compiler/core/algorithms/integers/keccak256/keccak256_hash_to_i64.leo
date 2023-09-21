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
    ) -> i64 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Keccak256 hash_to_i64 tests
        let a: i64 = Keccak256::hash_to_i64(addr_value);
        let b: i64 = Keccak256::hash_to_i64(bool_value);
        let c: i64 = Keccak256::hash_to_i64(field_value);
        let d: i64 = Keccak256::hash_to_i64(group_value);
        let e: i64 = Keccak256::hash_to_i64(i8_value);
        let f: i64 = Keccak256::hash_to_i64(i16_value);
        let g: i64 = Keccak256::hash_to_i64(i32_value);
        let h: i64 = Keccak256::hash_to_i64(i64_value);
        let i: i64 = Keccak256::hash_to_i64(i128_value);
        let j: i64 = Keccak256::hash_to_i64(u8_value);
        let k: i64 = Keccak256::hash_to_i64(u16_value);
        let l: i64 = Keccak256::hash_to_i64(u32_value);
        let m: i64 = Keccak256::hash_to_i64(u64_value);
        let n: i64 = Keccak256::hash_to_i64(u128_value);
        let o: i64 = Keccak256::hash_to_i64(scalar_value);
        // let p: i64 = Keccak256::hash_to_i64(string_value);
        let q: i64 = Keccak256::hash_to_i64(Foo { a: 1u128, b: 1u128 });
    
        return a;
    }
}
