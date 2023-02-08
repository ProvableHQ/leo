/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    function foo() -> field {
        return myGlobal;
    }
    
    function main() -> field {
        const myGlobal = 42field;
        const err = foo();
    
        return err;
    }
}
