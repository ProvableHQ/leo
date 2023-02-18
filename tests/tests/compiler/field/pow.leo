/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: field) -> bool {
        let negOneField: field = -1field;
        return negOneField ** 2field == 1field;
    }}
