function main(a: [[u8; 2]; 3]) {
    let b = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline

    console.assert(a == b);
}