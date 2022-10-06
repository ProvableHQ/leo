/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    // This record does define the `owner` variable but with the wrong type.
    record Token {
        gates: u64,
        owner: bool,
    }
    
    function main() -> bool {
        return true;
    }
}
