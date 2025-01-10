/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {    
    transition main() -> u8 {
        return (true + false) as u8;
    }
}
