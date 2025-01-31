/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {
    transition main(a: u8) -> u8 {
        return -a;
    }
}
