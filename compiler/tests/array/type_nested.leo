function main() {
    let a: [[u8; 3]; 2] = [[0; 3]; 2];

    let b: [[u8; 3]; 2] = [0; (2, 3)];

    console.assert(a == b);
}