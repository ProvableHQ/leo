/*
namespace: Parse
expectation: Fail
*/

program test.eth {
    transition doo(x: u32, y: i32) -> u32 {
        return 0u32;
    }
}