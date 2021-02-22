function main() {
    let b: [u8; (4, 3, 2)] = [[[0; 4]; 3]; 2]; // initializer (incorrectly reversed ordering)
}