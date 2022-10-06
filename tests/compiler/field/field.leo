/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/fields.in
*/

program test.aleo {    
    transition main(a: field) -> bool {
        const negOneField: field = -1field;
        return negOneField + a == 0field;
    }}
