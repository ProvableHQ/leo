/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar) -> group {
        let element: group = 0group;
    
        return element * a;
    }}
