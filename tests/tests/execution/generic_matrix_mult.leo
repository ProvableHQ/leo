/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main_1"
input = []
[case]
program = "test.aleo"
function = "main_2"
input = []
*/

const M_: u32 = 2u32; // Rows in A
const K_: u32 = 3u32; // Columns in A / Rows in B
const N_: u32 = 2u32; // Columns in B

struct Matrix::[M: u32, N: u32] {
    data: [[i32; N]; M]
}

// Multiplies two matrices wrapped in the `Matrix` struct
fn matmul::[M: u32, K: u32, N: u32](
    A: Matrix::[M, K],
    B: Matrix::[K, N]
) -> Matrix::[M, N] {
    let C_data: [[i32; N]; M] = [[0i32; N]; M];

    for i in 0u32..M {
        for j in 0u32..N {
            let sum = 0i32;
            for k in 0u32..K {
                sum += A.data[i][k] * B.data[k][j];
            }
            C_data[i][j] = sum;
        }
    }

    return Matrix::[M, N] { data: C_data };
}

// Checks if a square matrix is symmetric
fn is_symmetric::[N: u32](A: Matrix::[N, N]) -> bool {
    for i in 0u32..N-1 {
        for j in (i + 1u32)..N { // Only check upper triangle
            if A.data[i][j] != A.data[j][i] {
                return false;
            }
        }
    }
    return true;
}

program test.aleo {
    fn main_1() -> Matrix::[M_, N_] {
        // Matrix A: 2x3
        let A = Matrix::[M_, K_] {
            data: [
                [1i32, 2i32, 3i32],
                [4i32, 5i32, 6i32]
            ]
        };

        // Matrix B: 3x2
        let B = Matrix::[K_, N_] {
            data: [
                [7i32, 8i32],
                [9i32, 10i32],
                [11i32, 12i32]
            ]
        };

        // Compute matrix product: C = A * B
        let C = matmul::[M_, K_, N_](A, B);

        return C;
    }

    fn main_2() -> bool {
        let A = Matrix::[K_, K_] {
            data: [
                [1i32, 2i32, 3i32],
                [2i32, 4i32, 5i32],
                [3i32, 5i32, 6i32]
            ]
        };

        return is_symmetric::[3](A);
    }

    @noupgrade
    constructor() {}
}
