/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: field, b: field, y: bool) -> bool {
        return y == true && a == b;
    }
}
