/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u64, b: u64, c: bool) -> bool {
        return (a == b) == c;
    }
}
