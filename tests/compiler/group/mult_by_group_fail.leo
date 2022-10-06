/*
namespace: Compile
expectation: Fail
input_file: inputs/point.in
*/

program test.aleo {    
    function main(a: group) -> group {
        return (_, _)group * a;
    }}
