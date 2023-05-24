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
    
        // Pedersen64 commit to group tests
        let a: group = Pedersen64::commit_to_group(bool_value, 1scalar);
        let b: group = Pedersen64::commit_to_group(i8_value, 1scalar);
        let c: group = Pedersen64::commit_to_group(i16_value, 1scalar);
        let d: group = Pedersen64::commit_to_group(i32_value, 1scalar);
        let e: group = Pedersen64::commit_to_group(i64_value, 1scalar);
        let f: group = Pedersen64::commit_to_group(u8_value, 1scalar);
        let g: group = Pedersen64::commit_to_group(u16_value, 1scalar);
        let h: group = Pedersen64::commit_to_group(u32_value, 1scalar);
        let i: group = Pedersen64::commit_to_group(u64_value, 1scalar);
        // let j11: group = Pedersen64::commit_to_group(string_value, 1scalar);
    
        return a == i;
    }}
