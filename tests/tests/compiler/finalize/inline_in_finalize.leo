/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    inline foo(a: u8, b: u8) -> u8 {
        assert_neq(a, b);
        return a + b;
    }

    transition public_adder(public a: u8, public b: u8) {
        return then finalize(a, b);
    } finalize public_adder(a: u8, b: u8) {
        let result: u8 = foo(a, b);
        assert_neq(result, 0u8);
    }

}
