/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i64, b: i64, c: bool) -> bool{
        return (a != b) == c;
    }
}
