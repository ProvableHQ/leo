function main() {
    const a = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline

    const b: [u8; (3, 2)] = [0; (3, 2)]; // initializer

    console.assert(a == b);
}