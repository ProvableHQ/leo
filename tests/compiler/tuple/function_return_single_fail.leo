/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/bool_bool.in
*/

program test.aleo {    
    function main(a: bool, b: bool) -> (bool) {
        return (a);
    }}
