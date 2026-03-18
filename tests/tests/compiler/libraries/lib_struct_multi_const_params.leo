// Test: a library struct with two const generic parameters.
//
// Verifies that zip_eq of const_parameters and const_arguments in monomorphize_composite
// handles arity > 1 correctly, and that Matrix::[2u32, 3u32] and Matrix::[3u32, 4u32]
// are independently cached as distinct monomorphized composites.
//
// Dependency graph:
//   linalg  (library: struct Matrix::[R: u32, C: u32])
//   main.aleo (program: uses Matrix::[2, 3] and Matrix::[3, 4])

// --- library: linalg --- //

struct Matrix::[R: u32, C: u32] {
    rows: u32,
    cols: u32,
}

// --- Next Program --- //

program main.aleo {
    fn make_2x3() -> linalg::Matrix::[2u32, 3u32] {
        return linalg::Matrix::[2u32, 3u32] { rows: 2u32, cols: 3u32 };
    }

    fn make_3x4() -> linalg::Matrix::[3u32, 4u32] {
        return linalg::Matrix::[3u32, 4u32] { rows: 3u32, cols: 4u32 };
    }

    fn get_rows_2x3(m: linalg::Matrix::[2u32, 3u32]) -> u32 {
        return m.rows;
    }

    fn get_cols_3x4(m: linalg::Matrix::[3u32, 4u32]) -> u32 {
        return m.cols;
    }

    @noupgrade
    constructor() {}
}
