/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(a: u32, b: u32, y: bool) -> u32 {
        check_not_eq(a, b);
        if y {
           return adder(a, b);
        } else {
            return subber(a, b);
        }
    }

    inline check_not_eq(a: u32, b: u32) {
        assert(a != b);
    }

    inline adder(a: u32, b: u32) -> u32 {
        return a + b;
    }

    inline subber(a: u32, b: u32) -> u32 {
        return a - b;
    }
}
