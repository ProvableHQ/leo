/*
[case]
program = "test.aleo"
function = "main"
input = []
*/

// --- library: sizes --- //

const LEN: u32 = 4u32;
const HALF: u32 = LEN / 2u32;

// --- Next Program --- //

// Use library constants as const generic arguments and array sizes.

fn make_array::[N: u32]() -> [u32; N] {
    let arr: [u32; N] = [0u32; N];
    for i in 0u32..N {
        arr[i] = i + 1u32;
    }
    return arr;
}

fn sum_array::[N: u32](arr: [u32; N]) -> u32 {
    let total: u32 = 0u32;
    for i in 0u32..N {
        total += arr[i];
    }
    return total;
}

program test.aleo {
    fn main() -> u32 {
        // sizes::LEN = 4, sizes::HALF = 2

        // Array of size LEN (4): [1, 2, 3, 4] -> sum = 10
        let full: [u32; sizes::LEN] = make_array::[sizes::LEN]();
        let full_sum: u32 = sum_array::[sizes::LEN](full);

        // Array of size HALF (2): [1, 2] -> sum = 3
        let half: [u32; sizes::HALF] = make_array::[sizes::HALF]();
        let half_sum: u32 = sum_array::[sizes::HALF](half);

        // 10 + 3 = 13
        return full_sum + half_sum;
    }

    @noupgrade
    constructor() {}
}
