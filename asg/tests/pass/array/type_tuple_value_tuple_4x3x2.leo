function main() {
    const a = [[[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]]; // inline

    const b: [u8; (4, 3, 2)] = [0; (4, 3, 2)]; // initializer

    console.assert(a == b);
}