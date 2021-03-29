function main() {
    const a = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline

    const b: [[u8; 2]; 3] = [[0; 2]; 3]; // initializer

    console.assert(a == b);
}