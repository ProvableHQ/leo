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
    ) -> field {
        let bool_value: bool = true;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash tests
        let a: field = Pedersen64::hash(bool_value);
        let b: field = Pedersen64::hash(i8_value);
        let c: field = Pedersen64::hash(i16_value);
        let d: field = Pedersen64::hash(i32_value);
        let e: field = Pedersen64::hash(i64_value);
        let f: field = Pedersen64::hash(u8_value);
        let g: field = Pedersen64::hash(u16_value);
        let h: field = Pedersen64::hash(u32_value);
        let i: field = Pedersen64::hash(u64_value);
        // let j11: field = Pedersen64::hash(string_value);
    
        return a + i;
    }}
