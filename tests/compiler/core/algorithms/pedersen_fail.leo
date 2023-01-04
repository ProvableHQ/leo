/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() -> bool {
        let a: group = Pedersen64::hash(1u128); // Pedersen64 hash returns a field type
    
        return true;
    }}
