/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: field) -> bool {
        let negOneField: field = -1field;
        let negnegnegOneField: field = 0field - - - 1field;
        return negOneField + a == 0field;
    }}
