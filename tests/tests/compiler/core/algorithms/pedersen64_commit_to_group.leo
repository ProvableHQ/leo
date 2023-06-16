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

        // Pedersen64 commit_to_group tests
        let a: group = Pedersen64::commit_to_group(addr_value, scalar_value);
        let b: group = Pedersen64::commit_to_group(bool_value, scalar_value);
        let c: group = Pedersen64::commit_to_group(field_value, scalar_value);
        let d: group = Pedersen64::commit_to_group(group_value, scalar_value);
        let e: group = Pedersen64::commit_to_group(i8_value, scalar_value);
        let f: group = Pedersen64::commit_to_group(i16_value, scalar_value);
        let g: group = Pedersen64::commit_to_group(i32_value, scalar_value);
        let i: group = Pedersen64::commit_to_group(u8_value, scalar_value);
        let j: group = Pedersen64::commit_to_group(u16_value, scalar_value);
        let k: group = Pedersen64::commit_to_group(u32_value, scalar_value);
        let m: group = Pedersen64::commit_to_group(scalar_value, scalar_value);
        // let j11: group = Pedersen64::commit_to_group(string_value);
        let n: group = Pedersen64::commit_to_group(Foo { a: 1u16, b: 1u16 }, scalar_value);

        return a == i;
    }}
