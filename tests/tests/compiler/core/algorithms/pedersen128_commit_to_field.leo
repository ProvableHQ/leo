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

        // Pedersen128 commit_to_field tests
        let a: field = Pedersen128::commit_to_field(addr_value, scalar_value);
        let b: field = Pedersen128::commit_to_field(bool_value, scalar_value);
        let c: field = Pedersen128::commit_to_field(field_value, scalar_value);
        let d: field = Pedersen128::commit_to_field(group_value, scalar_value);
        let e: field = Pedersen128::commit_to_field(i8_value, scalar_value);
        let f: field = Pedersen128::commit_to_field(i16_value, scalar_value);
        let g: field = Pedersen128::commit_to_field(i32_value, scalar_value);
        let h: field = Pedersen128::commit_to_field(i64_value, scalar_value);
        let j: field = Pedersen128::commit_to_field(u8_value, scalar_value);
        let k: field = Pedersen128::commit_to_field(u16_value, scalar_value);
        let l: field = Pedersen128::commit_to_field(u32_value, scalar_value);
        let m: field = Pedersen128::commit_to_field(u64_value, scalar_value);
        let o: field = Pedersen128::commit_to_field(scalar_value, scalar_value);
        // let j11: field = Pedersen128::commit_to_field(string_value);
        let p: field = Pedersen128::commit_to_field(Foo { a: 1u32, b: 1u32 }, scalar_value);

        return a == j;
    }}
