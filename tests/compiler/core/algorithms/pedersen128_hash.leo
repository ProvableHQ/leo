/*
namespace: Compile
expectation: Pass
input_file: inputs/int64.in
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
    ) -> field {
        let bool_value: bool = true;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        // let string_value: string = "helloworld";
    
        // Pedersen128 hash tests
        let a: field = Pedersen128::hash(bool_value);
        let b: field = Pedersen128::hash(i8_value);
        let c: field = Pedersen128::hash(i16_value);
        let d: field = Pedersen128::hash(i32_value);
        let e: field = Pedersen128::hash(i64_value);
        let f: field = Pedersen128::hash(i128_value);
        let g: field = Pedersen128::hash(u8_value);
        let h: field = Pedersen128::hash(u16_value);
        let i: field = Pedersen128::hash(u32_value);
        let j: field = Pedersen128::hash(u64_value);
        let k: field = Pedersen128::hash(u128_value);
        // let l: field = Pedersen128::hash(string_value);
    
        return a + k;
    }}
