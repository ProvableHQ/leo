/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    inline adder(a: u8, b: u8) -> u8 {
        asm {
            add a b into c;
        }
        return c;
    }

    transition hello(a: u8, b: u8) -> u8 {
        return adder(a, b);
    }
}