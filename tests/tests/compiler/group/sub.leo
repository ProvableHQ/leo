/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: group, b: group, c: group) -> bool {
        assert(a - b == c);
    
        return a - b == c;
    }}
