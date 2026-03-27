program test.aleo {
    struct Matrix::[M: u32, N: u32] {
        values: [[u32; N]; M],
    }

    fn main() -> Matrix::[2u32, 3u32] {
        return Matrix::[2u32, 3u32] { values: [[1u32; 3u32]; 2u32] };
    }
}
