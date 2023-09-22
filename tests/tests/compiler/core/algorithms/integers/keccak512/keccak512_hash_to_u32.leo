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
    ) -> u32 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Keccak512 hash_to_u32 tests
        let a: u32 = Keccak512::hash_to_u32(addr_value);
        let b: u32 = Keccak512::hash_to_u32(bool_value);
        let c: u32 = Keccak512::hash_to_u32(field_value);
        let d: u32 = Keccak512::hash_to_u32(group_value);
        let e: u32 = Keccak512::hash_to_u32(i8_value);
        let f: u32 = Keccak512::hash_to_u32(i16_value);
        let g: u32 = Keccak512::hash_to_u32(i32_value);
        let h: u32 = Keccak512::hash_to_u32(i64_value);
        let i: u32 = Keccak512::hash_to_u32(i128_value);
        let j: u32 = Keccak512::hash_to_u32(u8_value);
        let k: u32 = Keccak512::hash_to_u32(u16_value);
        let l: u32 = Keccak512::hash_to_u32(u32_value);
        let m: u32 = Keccak512::hash_to_u32(u64_value);
        let n: u32 = Keccak512::hash_to_u32(u128_value);
        let o: u32 = Keccak512::hash_to_u32(scalar_value);
        // let p: u32 = Keccak512::hash_to_u32(string_value);
        let q: u32 = Keccak512::hash_to_u32(Foo { a: 1u128, b: 1u128 });
    
        return a;
    }
}
