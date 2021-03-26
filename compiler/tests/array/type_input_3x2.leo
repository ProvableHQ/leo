function main(a: [[u8; 2]; 3]) {
    const b = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline

    console.assert(a == b);
}