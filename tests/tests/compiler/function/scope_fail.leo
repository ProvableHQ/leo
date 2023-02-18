/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    function foo() -> field {
        return myGlobal;
    }
    
    function main() -> field {
        let myGlobal = 42field;
        let err = foo();
    
        return err;
    }
}
