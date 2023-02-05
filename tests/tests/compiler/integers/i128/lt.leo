/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i128, b: i128, c: bool) -> bool{
        return a < b == c;
    }
}
