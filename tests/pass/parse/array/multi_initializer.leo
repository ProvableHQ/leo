function main() {
    const a: [u8; (2, 2, 2)] = [1u8; (2, 2, 2)];

    const b: [u8; (2, 2, 2)] = [[[1u8; 2]; 2]; 2];

    console.assert(a == b);
}