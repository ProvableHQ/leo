function main() {
    let a: [u8; (2, 2, 2)] = [1u8; (2, 2, 2)];

    let b: [u8; (2, 2, 2)] = [[[1u8; 2]; 2]; 2];

    console.assert(a == b);
}