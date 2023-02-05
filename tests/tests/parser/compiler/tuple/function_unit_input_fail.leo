/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    function foo(a: ()) -> u8 {
        assert_eq(1u8, 2u8);
        return 3u8;
    }

    transition bar(a: u8, b: u8) -> u8 {
        foo(());
        return a + b;
    }

}
