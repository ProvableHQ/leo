/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    // This record does not define the `owner` variable required for a record type.
    record Token {
        // The token amount.
        amount: u64,
    }
    
    function main() -> bool {
        return true;
    }}
