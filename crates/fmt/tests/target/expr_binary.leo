program test.aleo {
    fn compute(a: u64, b: u64) -> u64 {
        let sum: u64 = a + b;
        let diff: u64 = a - b;
        let prod: u64 = a * b;
        let quot: u64 = a / b;
        let rem: u64 = a % b;
        let pow: u64 = a ** 2u32;
        let lt: bool = a < b;
        let gt: bool = a > b;
        let le: bool = a <= b;
        let ge: bool = a >= b;
        let eq: bool = a == b;
        let ne: bool = a != b;
        let and: bool = true && false;
        let or: bool = true || false;
        let band: u64 = a & b;
        let bor: u64 = a | b;
        let bxor: u64 = a ^ b;
        let shl: u64 = a << 2u8;
        let shr: u64 = a >> 2u8;
        return sum;
    }
}
