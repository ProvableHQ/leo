/*
namespace: Parse
expectation: Fail
*/

program test.aleo {    
    @foo(?, bar, ?)
    function x() -> u8 {
        return 1u8;
    }
    
    @bar(123) // ints not vali
    function x() -> u8 {
        return 2u8;
    }
    
    
    @context // recovery witness
    function x() -> u8 {
        return 3u8;
    }
}
