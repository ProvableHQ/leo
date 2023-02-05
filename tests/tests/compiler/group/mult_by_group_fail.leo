/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: group) -> group {
        return (_, _)group * a;
    }}
