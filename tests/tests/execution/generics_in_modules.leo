/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main"
input = []
*/

program test.aleo {
    fn main() -> u32 {
        let mat = common::use_matrix(); // Matrix with modified elements

        let outer = outer::OuterWrapper::[4] {
            nested: outer::inner::InnerGeneric::[4] {
                data: [0; 4],
            },
        };

        // ✅ Modify struct field using assignment
        for i in 0u32..4u32 {
            outer.nested.data[i] = i * 10;
        }

        // ✅ Modify nested matrix elements
        for i in 0u32..common::R {
            for j in 0u32..common::C {
                mat.inner.values[i][j] = mat.inner.values[i][j] + 1;
            }
        }

        return mat.inner.values[0][1] + outer.nested.data[3];
    }

    @noupgrade
    constructor() {}

}

// --- Next Module: common.leo --- //

const R: u32 = 2;
const C: u32 = 3;

// ✅ Generic struct representing a matrix
struct Matrix::[M: u32, N: u32] {
    values: [[u32; N]; M],
}

// ✅ Generic struct wrapping another generic struct (nested generics)
struct WrappedMatrix::[M: u32, N: u32] {
    inner: Matrix::[M, N],
}

// ✅ Function constructing matrix and modifying it with loop/assignment
fn use_matrix() -> WrappedMatrix::[R, C] {
    let base = Matrix::[R, C] {
        values: [[1; C]; R],
    };

    // ✅ Modify inner matrix values using loop and assignment
    for i in 0u32..R {
        for j in 0u32..C {
            base.values[i][j] = i + j;
        }
    }

    return WrappedMatrix::[R, C] {
        inner: base,
    };
}

// --- Next Module: outer.leo --- //

struct OuterWrapper::[N: u32] {
    nested: inner::InnerGeneric::[N],
}

// --- Next Module: outer/inner.leo --- //

struct InnerGeneric::[N: u32] {
    data: [u32; N],
}
