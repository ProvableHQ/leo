/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: bool) -> bool {
        return a == true;
    }
}
