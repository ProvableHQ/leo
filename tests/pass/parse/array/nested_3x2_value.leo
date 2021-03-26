// Multidimensional array syntax in leo
function main() {
    const a = [[0u32, 0u32], [0u32, 0u32], [0u32, 0u32]]; // inline

    const b: [u32; (3, 2)] = [[0; 2]; 3]; // initializer

    console.assert(a == b);
}