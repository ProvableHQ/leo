/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u16, b: u16, c: bool) -> bool{
        return (a != b) == c;
    }
}
