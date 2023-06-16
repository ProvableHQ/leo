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
    
        // BHP768 hash_to_address tests
        let a: address = BHP768::hash_to_address(addr_value);
        let b: address = BHP768::hash_to_address(bool_value);
        let c: address = BHP768::hash_to_address(field_value);
        let d: address = BHP768::hash_to_address(group_value);
        let e: address = BHP768::hash_to_address(i8_value);
        let f: address = BHP768::hash_to_address(i16_value);
        let g: address = BHP768::hash_to_address(i32_value);
        let h: address = BHP768::hash_to_address(i64_value);
        let i: address = BHP768::hash_to_address(i128_value);
        let j: address = BHP768::hash_to_address(u8_value);
        let k: address = BHP768::hash_to_address(u16_value);
        let l: address = BHP768::hash_to_address(u32_value);
        let m: address = BHP768::hash_to_address(u64_value);
        let n: address = BHP768::hash_to_address(u128_value);
        let o: address = BHP768::hash_to_address(scalar_value);
        // let p: address = BHP768::hash_to_address(string_value);
        let q: address = BHP768::hash_to_address(Foo { a: 1u128, b: 1u128 });
    
        return field_value;
    }
}
