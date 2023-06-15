/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    struct Foo {
        a: u16,
        b: u16,
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
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";

        // Pedersen64 commit_to_field tests
        let a: field = Pedersen64::commit_to_field(addr_value, scalar_value);
        let b: field = Pedersen64::commit_to_field(bool_value, scalar_value);
        let c: field = Pedersen64::commit_to_field(field_value, scalar_value);
        let d: field = Pedersen64::commit_to_field(group_value, scalar_value);
        let e: field = Pedersen64::commit_to_field(i8_value, scalar_value);
        let f: field = Pedersen64::commit_to_field(i16_value, scalar_value);
        let g: field = Pedersen64::commit_to_field(i32_value, scalar_value);
        let i: field = Pedersen64::commit_to_field(u8_value, scalar_value);
        let j: field = Pedersen64::commit_to_field(u16_value, scalar_value);
        let k: field = Pedersen64::commit_to_field(u32_value, scalar_value);
        let m: field = Pedersen64::commit_to_field(scalar_value, scalar_value);
        // let j11: field = Pedersen64::commit_to_address(string_value);
        let n: field = Pedersen64::commit_to_field(Foo { a: 1u16, b: 1u16 }, scalar_value);

        return a == i;
    }}
