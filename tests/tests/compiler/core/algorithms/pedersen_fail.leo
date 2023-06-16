/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() -> bool {
        let a: group = Pedersen64::hash_to_field(1u128); // Pedersen64 hash_to_field returns a field type
    
        return true;
    }}
