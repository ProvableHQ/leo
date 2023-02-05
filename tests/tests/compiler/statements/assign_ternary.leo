/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(x: u32) {
        let x: bool = true ? x: true;
    }}
