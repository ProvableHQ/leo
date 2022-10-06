/*
namespace: Parse
expectation: Pass
*/

program test.aleo {    
    // This test currently passes, but we should disallow dangling annotations.
    @test
    
    function test() -> u8 {}
}
