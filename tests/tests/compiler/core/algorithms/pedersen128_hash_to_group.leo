/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
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
        let bool_value: bool = true;
        // let string_value: string = "helloworld";
    
        // Pedersen128 hash_to_group tests
        let a: group = Pedersen128::hash_to_group(bool_value);
        let b: group = Pedersen128::hash_to_group(i8_value);
        let c: group = Pedersen128::hash_to_group(i16_value);
        let d: group = Pedersen128::hash_to_group(i32_value);
        let e: group = Pedersen128::hash_to_group(i64_value);
        let f: group = Pedersen128::hash_to_group(u8_value);
        let g: group = Pedersen128::hash_to_group(u16_value);
        let h: group = Pedersen128::hash_to_group(u32_value);
        let i: group = Pedersen128::hash_to_group(u64_value);
        // let j11: group = Pedersen128::hash(string_value);
    
        return a + i;
    }}
