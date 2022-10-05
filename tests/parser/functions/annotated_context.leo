/*
namespace: Parse
expectation: Pass
*/

program test.aleo {    
    @context
    function f() -> u8 {
        return 1u8;
    }
    
    @context // recovery witness
    function g() -> u8 {
        return 2u8;
    }
}
