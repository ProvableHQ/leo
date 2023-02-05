/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: scalar) -> scalar {
        return a.square_root(); // square root not supported for scalar types.
    }}
