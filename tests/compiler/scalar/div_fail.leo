/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/scalars.in
*/

program test.aleo {    
    function main(a: scalar, b: scalar) -> scalar {
        return a / b; // division not supported for scalar types.
    }}
