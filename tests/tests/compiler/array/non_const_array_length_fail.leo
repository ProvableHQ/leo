program test.aleo {
    transition main() -> u32 {
        let x = 2u32;
        let a: [u32; x] = [1u32];
        return a[0];
    }
}
