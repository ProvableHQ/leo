/*
namespace: Compile
expectation: Pass
input_file: inputs/int64.in
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
        // let group_value: group = (0, 1)group;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // BHP1024 hash tests
        let a: field = BHP1024::hash(addr_value);
        let b: field = BHP1024::hash(bool_value);
        let c: field = BHP1024::hash(field_value);
        // let d: field = BHP1024::hash(group_value);
        let e: field = BHP1024::hash(i8_value);
        let f: field = BHP1024::hash(i16_value);
        let g: field = BHP1024::hash(i32_value);
        let h: field = BHP1024::hash(i64_value);
        let i: field = BHP1024::hash(i128_value);
        let j: field = BHP1024::hash(u8_value);
        let k: field = BHP1024::hash(u16_value);
        let l: field = BHP1024::hash(u32_value);
        let m: field = BHP1024::hash(u64_value);
        let n: field = BHP1024::hash(u128_value);
        let o: field = BHP1024::hash(scalar_value);
        // let p: field = BHP1024::hash(string_value);
        let q: field = BHP256::hash(Foo { a: 1u128, b: 1u128 });
    
        return a + o;
    }}
