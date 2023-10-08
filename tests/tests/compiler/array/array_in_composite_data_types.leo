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

    transition foo(a: [[bool; 8]; 8]) -> bool {
        return a[0u32][0u32];
    }
}
