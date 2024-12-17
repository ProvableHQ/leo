/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {    
    transition main() -> u32 {
        return (true ? 1u8 : 1u16) as u32;
    }
}
