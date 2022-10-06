/*
namespace: Parse
expectation: Pass
*/

program test.aleo {    
    @foo
    @bar
    @baz
    function foo() -> u8 {}
    
    transition mint() -> u8 {}
    
    @test
    function test() -> u8 {}
    
    @program
    function foo() -> u8 {}
}
