/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: field, b: field, c: field) -> bool {
        return a / b != c;
    }}
