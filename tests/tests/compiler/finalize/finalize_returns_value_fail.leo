/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    async transition public_adder(public a: u8, public b: u8) -> Future {
        return finalize_public_adder(a, b);
    }

    async function finalize_public_adder(a: u8, b: u8) -> public u8 {
        return a + b;
    }
}