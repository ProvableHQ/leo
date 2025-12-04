/*
seed = 123456789

[case]
program = "test.aleo"
function = "main"
input = []
*/

program test.aleo {

    // Treat any 0-value as none for testing purposes
    transition main() -> (u64, u64) {
        let raw: [u64; 4] = [1, 0, 3, 0];
        // Convert to optionals: 0u64 → none, otherwise Some(value)
        let arr: [u64?; 4] = [
            raw[0] == 0u64 ? none : raw[0],
            raw[1] == 0u64 ? none : raw[1],
            raw[2] == 0u64 ? none : raw[2],
            raw[3] == 0u64 ? none : raw[3],
        ];

        let sum: u64 = 0u64;
        for i in 0u8..4u8 { // 1 + 2*2 + 3 + 4*4
            sum += arr[i].unwrap_or((i * i) as u64);
        }

        assert(sum == 14u64);

        return (sum, sum);
    }

    @noupgrade
    async constructor() {}
}
