/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    function foo(a: u8, b: u8) -> () {
        assert_eq(a, b);
    }

    transition main(a: u8, b: u8) -> u8 {
        foo(a, b);
        return a + b;
    }
}
