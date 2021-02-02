function main() {
    let arr: [u32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let expected: [u32; 2] = [0, 1];

    let actual = arr[..2];     // Should produce [0, 1]

    console.assert(expected == actual);
}