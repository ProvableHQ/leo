/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    struct Foo {
        a: u32,
        b: u32,
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
    ) -> u64 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Pedersen128 hash_to_u64 tests
        let a: u64 = Pedersen128::hash_to_u64(addr_value);
        let b: u64 = Pedersen128::hash_to_u64(bool_value);
        let c: u64 = Pedersen128::hash_to_u64(field_value);
        let d: u64 = Pedersen128::hash_to_u64(group_value);
        let e: u64 = Pedersen128::hash_to_u64(i8_value);
        let f: u64 = Pedersen128::hash_to_u64(i16_value);
        let g: u64 = Pedersen128::hash_to_u64(i32_value);
        let h: u64 = Pedersen128::hash_to_u64(i64_value);
        let j: u64 = Pedersen128::hash_to_u64(u8_value);
        let k: u64 = Pedersen128::hash_to_u64(u16_value);
        let l: u64 = Pedersen128::hash_to_u64(u32_value);
        let m: u64 = Pedersen128::hash_to_u64(u64_value);
        let o: u64 = Pedersen128::hash_to_u64(scalar_value);
        // let p: u64 = Pedersen128::hash_to_u64(string_value);
        let q: u64 = Pedersen128::hash_to_u64(Foo { a: 1u32, b: 1u32, });
    
        return a;
    }
}
