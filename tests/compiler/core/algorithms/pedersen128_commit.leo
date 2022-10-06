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
    ) -> group {
        let bool_value: bool = true;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash tests
        let a: group = Pedersen128::commit(bool_value, 1scalar);
        let b: group = Pedersen128::commit(i8_value, 1scalar);
        let c: group = Pedersen128::commit(i16_value, 1scalar);
        let d: group = Pedersen128::commit(i32_value, 1scalar);
        let e: group = Pedersen128::commit(i64_value, 1scalar);
        let f: group = Pedersen128::commit(u8_value, 1scalar);
        let g: group = Pedersen128::commit(u16_value, 1scalar);
        let h: group = Pedersen128::commit(u32_value, 1scalar);
        let i: group = Pedersen128::commit(u64_value, 1scalar);
        // let j11: group = Pedersen128::commit(string_value, 1scalar);
    
        return a + i;
    }}
