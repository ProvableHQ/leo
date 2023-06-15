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
    ) -> address {
        let bool_value: bool = true;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash_to_address tests
        let a: address = Pedersen64::hash_to_address(bool_value);
        let b: address = Pedersen64::hash_to_address(i8_value);
        let c: address = Pedersen64::hash_to_address(i16_value);
        let d: address = Pedersen64::hash_to_address(i32_value);
        let e: address = Pedersen64::hash_to_address(i64_value);
        let f: address = Pedersen64::hash_to_address(u8_value);
        let g: address = Pedersen64::hash_to_address(u16_value);
        let h: address = Pedersen64::hash_to_address(u32_value);
        let i: address = Pedersen64::hash_to_address(u64_value);
        // let j11: address = Pedersen64::hash_to_address(string_value);
    
        return a;
    }
}
