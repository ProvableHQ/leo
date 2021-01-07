function main() {
    let a = [0u32; 4];

    let b = a[0..2]; // This should not cause a type inference error
}