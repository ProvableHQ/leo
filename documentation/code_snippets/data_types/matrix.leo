// ANCHOR: file
struct Matrix::[N: u32, M: u32] {
    data: [field; N * M],
}

program matrix_demo.aleo {
    fn build() -> Matrix::[2u32, 2u32] {
        // Usage
        let m = Matrix::[2u32, 2u32] { data: [0field, 1field, 2field, 3field] };
        return m;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
