program test.aleo {
    transition main() -> u32 {
        let sum: u32 = 0u32;
        for i: u32 in 0u32..5u32 {
            sum = sum + i;
        }
        return sum;
    }
}
