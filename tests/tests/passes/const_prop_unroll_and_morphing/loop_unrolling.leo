program test.aleo {
    fn main() -> u32 {
        let sum = 0u32;
        for i in 0u32..5u32 {
            sum += i;
        }
        sum += 1;
        for i in 0u32..0u32 {
            sum += i;
        }
        sum += 1;
        for i in 5u32..0u32 {
            sum += i;
        }
        return sum;
    }
}

