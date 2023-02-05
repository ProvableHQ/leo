/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() {
        const a: i32 = 2147483648i32;
    }
}
