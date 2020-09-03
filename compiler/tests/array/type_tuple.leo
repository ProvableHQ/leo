function main() {
    let a: [u8; (2, 3)] = [[0; 3]; 2];

    let b: [u8; (2, 3)] = [0; (2, 3)];

    console.assert(a == b);
}