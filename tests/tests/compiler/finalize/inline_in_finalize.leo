/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    inline foo(a: u8, b: u8) -> u8 {
        return a + b;
    }

    transition public_adder(public a: u8, public b: u8) {
        return then finalize(a, b);
    } finalize public_adder(a: u8, b: u8) -> public u8 {
        return foo(a, b);
    }

}
