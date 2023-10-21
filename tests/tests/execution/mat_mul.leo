/*
namespace: Execute
expectation: Pass
cases:
    matrix_multiply:
    - input:
       - "[1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64]"
       - "[1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64]"
*/


program test.aleo {
    transition matrix_multiply(a: [[u64; 3]; 3], b: [[u64; 3]; 3]) -> [[u64; 3]; 3] {
        let mut result: [[u64; 3]; 3] = [[0u64; 3]; 3];
        for i: u8 in 0u8..3u8 {
            for j: u8 in 0u8..3u8 {
                for k: u8 in 0u8..3u8 {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        return result;
    }
}
