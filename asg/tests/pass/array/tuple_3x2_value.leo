// Multidimensional array syntax in leo
function main() {
    let a = [[0u32, 0u32], [0u32, 0u32], [0u32, 0u32]]; // inline

    let b: [u32; (3, 2)] = [0; (3, 2)]; // initializer

    console.assert(a == b);
}