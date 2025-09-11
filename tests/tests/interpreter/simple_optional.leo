/*
seed = 987654321

[case]
program = "optional_sum.aleo"
function = "sum_with_defaults"
input = ["[1u64, 2u64, 3u64, 4u64]"]

[case]
program = "optional_sum.aleo"
function = "sum_with_defaults"
input = ["[10u64, 0u64, 30u64, 0u64]"]

[case]
program = "optional_sum.aleo"
function = "sum_with_defaults"
input = ["[0u64, 0u64, 0u64, 0u64]"]
*/

// TODO: FIX - array values can't have None currently

program optional_sum.aleo {

    // Treat any 0-value as None for testing purposes
    transition sum_with_defaults(raw: [u64; 4]) -> u64 {
        // Convert to optionals: 0u64 → None, otherwise Some(value)
        let arr: [u64?; 4] = [
            raw[0] == 0u64 ? None : raw[0],
            raw[1] == 0u64 ? None : raw[1],
            raw[2] == 0u64 ? None : raw[2],
            raw[3] == 0u64 ? None : raw[3],
        ];

        let sum: u64 = 0u64;
        for i in 0u8..4u8 {
            sum += arr[i].unwrap_or(0u64);
        }

        return sum;
    }

    @noupgrade
    async constructor() {}
}
