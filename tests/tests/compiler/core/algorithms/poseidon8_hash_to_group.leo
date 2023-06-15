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
    ) -> group {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";

        // Poseidon8 hash_to_group tests
        let a: group = Poseidon8::hash_to_group(addr_value);
        let b: group = Poseidon8::hash_to_group(bool_value);
        let c: group = Poseidon8::hash_to_group(field_value);
        let d: group = Poseidon8::hash_to_group(group_value);
        let e: group = Poseidon8::hash_to_group(i8_value);
        let f: group = Poseidon8::hash_to_group(i16_value);
        let g: group = Poseidon8::hash_to_group(i32_value);
        let h: group = Poseidon8::hash_to_group(i64_value);
        let i: group = Poseidon8::hash_to_group(i128_value);
        let j: group = Poseidon8::hash_to_group(u8_value);
        let k: group = Poseidon8::hash_to_group(u16_value);
        let l: group = Poseidon8::hash_to_group(u32_value);
        let m: group = Poseidon8::hash_to_group(u64_value);
        let n: group = Poseidon8::hash_to_group(u128_value);
        let o: group = Poseidon8::hash_to_group(scalar_value);
        // let p: group = Poseidon8::hash_to_group(string_value);
        let q: group = Poseidon8::hash_to_group(Foo { a: 1u128, b: 1u128 });

        return a + o;
    }}
