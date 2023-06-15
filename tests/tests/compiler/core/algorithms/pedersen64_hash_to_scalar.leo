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
    ) -> scalar {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash_to_scalar tests
        let a: scalar = Pedersen64::hash_to_scalar(addr_value);
        let b: scalar = Pedersen64::hash_to_scalar(bool_value);
        let c: scalar = Pedersen64::hash_to_scalar(field_value);
        let d: scalar = Pedersen64::hash_to_scalar(group_value);
        let e: scalar = Pedersen64::hash_to_scalar(i8_value);
        let f: scalar = Pedersen64::hash_to_scalar(i16_value);
        let g: scalar = Pedersen64::hash_to_scalar(i32_value);
        let i: scalar = Pedersen64::hash_to_scalar(u8_value);
        let j: scalar = Pedersen64::hash_to_scalar(u16_value);
        let k: scalar = Pedersen64::hash_to_scalar(u32_value);
        let m: scalar = Pedersen64::hash_to_scalar(scalar_value);
        // let j11: scalar = Pedersen64::hash_to_scalar(string_value);
        let n: scalar = Pedersen64::hash_to_scalar(Foo { a: 1u16, b: 1u16 });

        return a;
    }
}
