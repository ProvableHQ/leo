/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u8, b: u8, c: bool) -> bool{
        return a < b == c;
    }
}
