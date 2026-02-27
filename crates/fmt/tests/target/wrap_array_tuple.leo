program test.aleo {
    fn main(public a: u64, public b: u64) -> u64 {
        let short_arr: [u64; 3] = [1u64, 2u64, 3u64];
        let first_amount: u64 = a + 1u64;
        let second_amount: u64 = b + 2u64;
        let third_amount: u64 = a + 3u64;
        let fourth_amount: u64 = b + 4u64;
        let fifth_amount: u64 = a + 5u64;
        let long_arr: [u64; 5] = [
            first_amount,
            second_amount,
            third_amount,
            fourth_amount,
            fifth_amount,
        ];
        let repeat: [u64; 4] = [0u64; 4u64];
        let short_tup: (u64, u64) = (a, b);
        let long_tup: (u64, u64, u64, u64, u64) = (
            first_amount,
            second_amount,
            third_amount,
            fourth_amount,
            fifth_amount,
        );
        let commented_arr: [u64; 3] = [
            first_amount,
            second_amount,
            third_amount,
            // array trailing comment
        ];
        return short_arr[0u32] + long_arr[0u32] + commented_arr[0u32];
    }
}
