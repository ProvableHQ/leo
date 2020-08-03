// Multidimensional array syntax in leo
function main() {
    const a = [[0u32, 0u32, 0u32], [0u32, 0u32, 0u32]]; // inline

    const b: u32[3][2] = [[0; 3]; 2]; // initializer

    assert_eq!(a, b);
}