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
    ) -> i16 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Keccak256 hash_to_i16 tests
        let a: i16 = Keccak256::hash_to_i16(addr_value);
        let b: i16 = Keccak256::hash_to_i16(bool_value);
        let c: i16 = Keccak256::hash_to_i16(field_value);
        let d: i16 = Keccak256::hash_to_i16(group_value);
        let e: i16 = Keccak256::hash_to_i16(i8_value);
        let f: i16 = Keccak256::hash_to_i16(i16_value);
        let g: i16 = Keccak256::hash_to_i16(i32_value);
        let h: i16 = Keccak256::hash_to_i16(i64_value);
        let i: i16 = Keccak256::hash_to_i16(i128_value);
        let j: i16 = Keccak256::hash_to_i16(u8_value);
        let k: i16 = Keccak256::hash_to_i16(u16_value);
        let l: i16 = Keccak256::hash_to_i16(u32_value);
        let m: i16 = Keccak256::hash_to_i16(u64_value);
        let n: i16 = Keccak256::hash_to_i16(u128_value);
        let o: i16 = Keccak256::hash_to_i16(scalar_value);
        // let p: i16 = Keccak256::hash_to_i16(string_value);
        let q: i16 = Keccak256::hash_to_i16(Foo { a: 1u128, b: 1u128 });
    
        return a;
    }
}
