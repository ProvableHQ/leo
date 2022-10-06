/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/scalars.in
*/

program test.aleo {    
    function main(a: scalar) -> scalar {
        return a.square_root(); // square root not supported for scalar types.
    }}
