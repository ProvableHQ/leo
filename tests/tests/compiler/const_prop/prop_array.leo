
program test.aleo {
    transition main() -> u8 {
        const a: [u8; 3] = [1u8, 2u8, 3u8];
        return a[2u8];
    }
}
