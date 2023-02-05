/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i8, b: i8, c: bool) -> bool {
        return (a == b) == c;
    }
}
