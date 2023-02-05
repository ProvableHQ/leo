/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: scalar, b: scalar) -> scalar {
        return a / b; // division not supported for scalar types.
    }}
