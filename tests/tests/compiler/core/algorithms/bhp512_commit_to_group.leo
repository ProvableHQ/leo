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
    
        // BHP512 commit tests
        let a: group = BHP512::commit_to_group(addr_value, 1scalar);
        let b: group = BHP512::commit_to_group(bool_value, 1scalar);
        let c: group = BHP512::commit_to_group(field_value, 1scalar);
        let d: group = BHP512::commit_to_group(group_value, 1scalar);
        let e: group = BHP512::commit_to_group(i8_value, 1scalar);
        let f: group = BHP512::commit_to_group(i16_value, 1scalar);
        let g: group = BHP512::commit_to_group(i32_value, 1scalar);
        let h: group = BHP512::commit_to_group(i64_value, 1scalar);
        let i: group = BHP512::commit_to_group(i128_value, 1scalar);
        let j: group = BHP512::commit_to_group(u8_value, 1scalar);
        let k: group = BHP512::commit_to_group(u16_value, 1scalar);
        let l: group = BHP512::commit_to_group(u32_value, 1scalar);
        let m: group = BHP512::commit_to_group(u64_value, 1scalar);
        let n: group = BHP512::commit_to_group(u128_value, 1scalar);
        let o: group = BHP512::commit_to_group(scalar_value, 1scalar);
        // let p: group = BHP512::commit_to_group(string_value, 1scalar);
        let q: group = BHP512::commit_to_group(Foo { a: 1u128, b: 2u128 }, 1scalar);
    
        return a + o;
    }}
