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
    ) -> bool {
        let bool_value: bool = true;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash tests
        let a: field = Pedersen64::commit(bool_value, 1scalar);
        let b: field = Pedersen64::commit(i8_value, 1scalar);
        let c: field = Pedersen64::commit(i16_value, 1scalar);
        let d: field = Pedersen64::commit(i32_value, 1scalar);
        let e: field = Pedersen64::commit(i64_value, 1scalar);
        let f: field = Pedersen64::commit(u8_value, 1scalar);
        let g: field = Pedersen64::commit(u16_value, 1scalar);
        let h: field = Pedersen64::commit(u32_value, 1scalar);
        let i: field = Pedersen64::commit(u64_value, 1scalar);
        // let j11: field = Pedersen64::commit(string_value, 1scalar);
    
        return a == i;
    }}
