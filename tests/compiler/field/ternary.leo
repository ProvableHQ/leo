/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/fields.in
*/

program test.aleo {    
    transition main(a: field, b: field, c: field) -> bool {
        return b == 1field ? a == 1field : c == 2field;
    }}
