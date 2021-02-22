function main() {
    let b: [[[u8; 2]; 3]; 4] = [0; (2, 3, 4)]; // initializer (incorrectly reversed ordering)
}