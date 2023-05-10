/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token {
        // The token owner.
        owner: address,
        // The token owner.
        owner: address, // Cannot define two record variables with the same name.
        // The token amount.
        amount: u64,
    }
    
    function main() -> bool {
        return true;
    }}
