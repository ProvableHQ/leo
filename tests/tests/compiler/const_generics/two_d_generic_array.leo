const M_: u32 = 2u32; // Rows in A
const K_: u32 = 3u32; // Columns in A / Rows in B
const N_: u32 = 2u32; // Columns in B

program test.aleo {
    fn main() -> [[i32; N_]; M_] {
        // Matrix A: 2x3
        let A: [[i32; K_]; M_] = [
            [1i32, 2i32, 3i32],
            [4i32, 5i32, 6i32]
        ];

        // Matrix B: 3x2
        let B: [[i32; N_]; K_] = [
            [7i32, 8i32],
            [9i32, 10i32],
            [11i32, 12i32]
        ];

        // Compute matrix product: C = A * B
        let C = matmul::[M_, K_, N_](A, B);

        return C;
    }
}

// Multiply two 2D matrices: A[M][K] * B[K][N] = C[M][N]
fn matmul::[M: u32, K: u32, N: u32](
    A: [[i32; K]; M],
    B: [[i32; N]; K]
) -> [[i32; N]; M] {
    let C: [[i32; N]; M] = [[0i32; N]; M];

    for i in 0u32..M {
        for j in 0u32..N {
            let sum = 0i32;
            for k in 0u32..K {
                sum += A[i][k] * B[k][j];
            }
            C[i][j] = sum;
        }
    }

    return C;
}
