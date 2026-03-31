program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        let sum = a + b;
        let diff = a - b;

        assert_eq(sum, diff);

        // Return the computed sum.
        return sum;
    }
}
