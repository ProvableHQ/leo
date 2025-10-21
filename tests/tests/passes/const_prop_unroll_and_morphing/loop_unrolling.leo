program test.aleo {
    transition main() -> u32 {
        let sum = 0u32;
        for i in 0u32..5u32 {
            sum += i;
        }
        return sum;
    }
}

