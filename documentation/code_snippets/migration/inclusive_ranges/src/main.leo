program ranges_demo.aleo {
    fn sum_to_ten() -> u32 {
        let total: u32 = 0u32;
        // ANCHOR: snippet
        for i: u32 in 0u32..=10u32 {
            // i takes values 0, 1, 2, ..., 10
            total += i;
        }
        // ANCHOR_END: snippet
        return total;
    }

    @noupgrade
    constructor() {}
}
