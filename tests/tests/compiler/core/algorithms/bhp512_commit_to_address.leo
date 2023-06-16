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
//         let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // BHP512 commit_to_address tests
        let a: address = BHP512::commit_to_address(addr_value, 1scalar);
        let b: address = BHP512::commit_to_address(bool_value, 1scalar);
        let c: address = BHP512::commit_to_address(field_value, 1scalar);
//         let d: address = BHP512::commit_to_address(group_value, 1scalar);
        let e: address = BHP512::commit_to_address(i8_value, 1scalar);
        let f: address = BHP512::commit_to_address(i16_value, 1scalar);
        let g: address = BHP512::commit_to_address(i32_value, 1scalar);
        let h: address = BHP512::commit_to_address(i64_value, 1scalar);
        let i: address = BHP512::commit_to_address(i128_value, 1scalar);
        let j: address = BHP512::commit_to_address(u8_value, 1scalar);
        let k: address = BHP512::commit_to_address(u16_value, 1scalar);
        let l: address = BHP512::commit_to_address(u32_value, 1scalar);
        let m: address = BHP512::commit_to_address(u64_value, 1scalar);
        let n: address = BHP512::commit_to_address(u128_value, 1scalar);
        let o: address = BHP512::commit_to_address(scalar_value, 1scalar);
        // let p: address = BHP512::commit_to_address(string_value, 1scalar);
        let q: address = BHP512::commit_to_address(Foo { a: 1u128, b: 2u128 }, 1scalar);
    
        return field_value;
    }}
