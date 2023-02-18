/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() {
        let a: i64 = 9223372036854775808i64;
    }
}
