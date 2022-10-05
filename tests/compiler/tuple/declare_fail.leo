/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/bool_bool.in
*/

program test.aleo {    
    function main(a: bool, b: bool) -> (bool, bool) {
        let t: (bool, bool) = (a, 1u64); // We should be declaring to a boolean, not a u64.
    
        return (t.0, t.1);
    }}
