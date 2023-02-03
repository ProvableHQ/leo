/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u32, b: u32, c: bool) -> bool {
        return (a == b) == c;
    }
}
