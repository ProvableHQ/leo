/*
namespace = "Compile"
expectation = "Pass"
*/

// This test should pass, but produce warnings about unrecognized and malformed annotations.

program test.aleo {    
    @foo
    function foo(a: u8, b: u8) -> u8 {
        return a + b;
    }
    
    @program
    function bar(a: u8, b: u8) -> u8 {
        return a * b;
    }

    @test(foo)
    transition test() {
        assert(1u32 == 1u32);
    }
}
