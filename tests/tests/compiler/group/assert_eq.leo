/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: group, b: group) -> bool {
        assert(a == b);
        return a == b;
    }}
