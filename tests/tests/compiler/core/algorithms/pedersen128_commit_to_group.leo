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
    ) -> bool {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";

        // Pedersen128 commit_to_group tests
        let a: group = Pedersen128::commit_to_group(addr_value, scalar_value);
        let b: group = Pedersen128::commit_to_group(bool_value, scalar_value);
        let c: group = Pedersen128::commit_to_group(field_value, scalar_value);
        let d: group = Pedersen128::commit_to_group(group_value, scalar_value);
        let e: group = Pedersen128::commit_to_group(i8_value, scalar_value);
        let f: group = Pedersen128::commit_to_group(i16_value, scalar_value);
        let g: group = Pedersen128::commit_to_group(i32_value, scalar_value);
        let h: group = Pedersen128::commit_to_group(i64_value, scalar_value);
        let j: group = Pedersen128::commit_to_group(u8_value, scalar_value);
        let k: group = Pedersen128::commit_to_group(u16_value, scalar_value);
        let l: group = Pedersen128::commit_to_group(u32_value, scalar_value);
        let m: group = Pedersen128::commit_to_group(u64_value, scalar_value);
        let o: group = Pedersen128::commit_to_group(scalar_value, scalar_value);
        // let j11: group = Pedersen128::commit_to_group(string_value);
        let p: group = Pedersen128::commit_to_group(Foo { a: 1u32, b: 1u32 }, scalar_value);

        return a == j;
    }}
