/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() {
        let a: u64 = 18446744073709551616u64;
    }
}
