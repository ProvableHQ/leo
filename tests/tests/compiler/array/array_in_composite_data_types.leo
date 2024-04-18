/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    struct bar {
        data: [u8; 8],
    }

    record floo {
        owner: address,
        data: [u8; 8],
    }

    transition foo(a: [[bool; 8]; 8]) -> (bool, bar, floo) {
        return (a[0u32][0u32], bar { data: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8] }, floo { owner: self.caller, data: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8] });
    }
}
