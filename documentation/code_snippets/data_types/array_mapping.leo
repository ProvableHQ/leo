// ANCHOR: file
program array_mapping.aleo {
    // Declare a mapping that contains array values
    mapping data: address => [bool; 8];

    // Iterate over an array using a for loop and sum the values within
    fn sum_with_loop(a: [u64; 4]) -> u64 {
        let sum: u64 = 0u64;
        for i: u8 in 0u8..4u8 {
            sum += a[i];
        }
        return sum;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
