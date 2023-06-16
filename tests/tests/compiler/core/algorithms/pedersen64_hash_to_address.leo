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
    ) -> address {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash_to_address tests
        let a: address = Pedersen64::hash_to_address(addr_value);
        let b: address = Pedersen64::hash_to_address(bool_value);
        let c: address = Pedersen64::hash_to_address(field_value);
        let d: address = Pedersen64::hash_to_address(group_value);
        let e: address = Pedersen64::hash_to_address(i8_value);
        let f: address = Pedersen64::hash_to_address(i16_value);
        let g: address = Pedersen64::hash_to_address(i32_value);
        let i: address = Pedersen64::hash_to_address(u8_value);
        let j: address = Pedersen64::hash_to_address(u16_value);
        let k: address = Pedersen64::hash_to_address(u32_value);
        let m: address = Pedersen64::hash_to_address(scalar_value);
        // let j11: address = Pedersen64::hash_to_address(string_value);
        let n: address = Pedersen64::hash_to_address(Foo { a: 1u16, b: 1u16 });

        return a;
    }
}
