/*
seed = 123456789
min_height = 16

[case]
program = "bubble_sort.aleo"
function = "sort"
input = ["[100u32, 0u32, 0u32, 2u32, 5u32, 200u32, 300u32, 10u32]"]
[case]
program = "bubble_sort.aleo"
function = "sort"
input = ["[1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32]"]
[case]
program = "bubble_sort.aleo"
function = "sort"
input = ["[8u32, 7u32, 6u32, 5u32, 4u32, 3u32, 2u32, 1u32]"]
*/


program bubble_sort.aleo {
    // This is largely to test array assignments.
    transition sort(xs: [u32; 8]) -> public [u32; 8] {
        for i: u8 in 0u8..7u8 {
            for j: u8 in 0u8..7u8-i {
                if xs[j+1u8] < xs[j] {
                    let swap: u32 = xs[j];
                    xs[j] = xs[j+1u8];
                    xs[j+1u8] = swap;
                }
            }
        }
        return xs;
    }

    @noupgrade
    async constructor() {}
}
