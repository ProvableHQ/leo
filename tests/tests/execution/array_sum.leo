/*
namespace: Execute
expectation: Pass
cases:
    sum_manually:
    - input: ["[1u64, 2u64, 3u64, 4u64]"]
    sum_with_loop:
    - input: ["[1u64, 2u64, 3u64, 4u64]"]
*/


program test.aleo {
    transition sum_manually(a: [u64; 4]) -> u64 {
        return a[0] + a[1] + a[2] + a[3];
    }

    transition sum_with_loop(a: [u64; 4]) -> u64 {
        let sum: u64 = 0u64;
        for i: u8 in 0u8..4u8 {
            sum += a[i];
        }
        return sum;
    }
}
