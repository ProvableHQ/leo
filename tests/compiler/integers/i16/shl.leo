/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

program test.aleo {    
    transition main(a: i16, b: i16, c: i16) -> bool {
        return a << 2u8 == a << 2u16 && a << 2u32 == c;
    }
}
