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
    ) -> scalar {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // SHA3_384 hash_to_scalar tests
        let a: scalar = SHA3_384::hash_to_scalar(addr_value);
        let b: scalar = SHA3_384::hash_to_scalar(bool_value);
        let c: scalar = SHA3_384::hash_to_scalar(field_value);
        let d: scalar = SHA3_384::hash_to_scalar(group_value);
        let e: scalar = SHA3_384::hash_to_scalar(i8_value);
        let f: scalar = SHA3_384::hash_to_scalar(i16_value);
        let g: scalar = SHA3_384::hash_to_scalar(i32_value);
        let h: scalar = SHA3_384::hash_to_scalar(i64_value);
        let i: scalar = SHA3_384::hash_to_scalar(i128_value);
        let j: scalar = SHA3_384::hash_to_scalar(u8_value);
        let k: scalar = SHA3_384::hash_to_scalar(u16_value);
        let l: scalar = SHA3_384::hash_to_scalar(u32_value);
        let m: scalar = SHA3_384::hash_to_scalar(u64_value);
        let n: scalar = SHA3_384::hash_to_scalar(u128_value);
        let o: scalar = SHA3_384::hash_to_scalar(scalar_value);
        // let p: scalar = SHA3_384::hash_to_scalar(string_value);
        let q: scalar = SHA3_384::hash_to_scalar(Foo { a: 1u128, b: 1u128 });
    
        return a + o;
    }}
