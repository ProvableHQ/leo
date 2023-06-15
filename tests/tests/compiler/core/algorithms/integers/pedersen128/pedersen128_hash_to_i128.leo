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
    ) -> i128 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Pedersen128 hash_to_i128 tests
        let a: i128 = Pedersen128::hash_to_i128(addr_value);
        let b: i128 = Pedersen128::hash_to_i128(bool_value);
        let c: i128 = Pedersen128::hash_to_i128(field_value);
        let d: i128 = Pedersen128::hash_to_i128(group_value);
        let e: i128 = Pedersen128::hash_to_i128(i8_value);
        let f: i128 = Pedersen128::hash_to_i128(i16_value);
        let g: i128 = Pedersen128::hash_to_i128(i32_value);
        let h: i128 = Pedersen128::hash_to_i128(i64_value);
        let j: i128 = Pedersen128::hash_to_i128(u8_value);
        let k: i128 = Pedersen128::hash_to_i128(u16_value);
        let l: i128 = Pedersen128::hash_to_i128(u32_value);
        let m: i128 = Pedersen128::hash_to_i128(u64_value);
        let o: i128 = Pedersen128::hash_to_i128(scalar_value);
        // let p: i128 = Pedersen128::hash_to_i128(string_value);
        let q: i128 = Pedersen128::hash_to_i128(Foo { a: 1u32, b: 1u32, });
    
        return a;
    }
}
