/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: group) -> bool {
        return a == (0, 1)group;
    }}
