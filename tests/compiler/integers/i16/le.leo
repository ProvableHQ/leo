/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i16, b: i16, c: bool) -> bool {
        return a <= b == c;
    }
}
