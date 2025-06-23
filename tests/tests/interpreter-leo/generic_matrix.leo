program test.aleo {
    const M_: u32 = 2u32; // Rows in A
    const K_: u32 = 3u32; // Columns in A / Rows in B
    const N_: u32 = 2u32; // Columns in B

    struct Matrix::[M: u32, N: u32] {
        data: [[i32; N]; M]
    }

    transition main() -> bool {
        // Matrix A: 2x3
        let A = Matrix::[M_, K_] {
            data: [
                [1, 2, 3],
                [4, 5, 6]
            ]
        };

        // Matrix B: 3x2
        let B = Matrix::[K_, N_] {
            data: [
                [7, 8],
                [9, 10],
                [11, 12]
            ]
        };

        // Compute matrix product: C = A *  and check it
        matmul_and_check::[M_, K_, N_](A, B);

        // If none of the asserts fail, the output will just be `true`
        return true;
    }

    // Multiply two matrices wrapped in the `Matrix` struct
    inline matmul_and_check::[M: u32, K: u32, N: u32](
        A: Matrix::[M, K],
        B: Matrix::[K, N]
    ) {
        let C_data: [[i32; N]; M] = [[0; N]; M];

        for i in 0u32..M {
            for j in 0u32..N {
                let sum = 0i32;
                for k in 0u32..K {
                    sum += A.data[i][k] * B.data[k][j];
                }
                // This is just a workaround due to the fact that we can't 
                // assign to `C[i][j]` just yet in the interpreter
                if i == 0 && j == 0 {
                    assert(sum == 58);
                } else if i == 0 && j == 1 {
                    assert(sum == 64);
                } else if i == 1 && j == 0 {
                    assert(sum == 139);
                } else if i == 1 && j == 1 {
                    assert(sum == 154);
                }
            }
        }
    }
}
