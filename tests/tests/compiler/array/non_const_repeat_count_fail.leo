program test.aleo {
    transition main(x: u32) -> u32 {
        let a = [1u32; x];
        return a[0];
    }
}
