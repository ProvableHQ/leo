/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() -> bool {
        let a: group = Pedersen64::hash(1u128);
    
        return true;
    }}
