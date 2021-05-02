function main() {
    const x = 0u8;
    const a = [0u8; 4];
    console.assert(a[x] == 0);
}
