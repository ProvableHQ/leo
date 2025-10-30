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

// --- Test --- //

import bubble_sort.aleo;
program test_bubble_sort.aleo {

    @test
    transition sort_1() {
        // input = [100, 0, 0, 2, 5, 200, 300, 10]
        assert_eq(
            [0u32, 0u32, 2u32, 5u32, 10u32, 100u32, 200u32, 300u32],
            bubble_sort.aleo/sort([100u32, 0u32, 0u32, 2u32, 5u32, 200u32, 300u32, 10u32])
        );
    }

    @test
    transition sort_2() {
        // input = [1, 2, 3, 4, 5, 6, 7, 8] (already sorted)
        assert_eq(
            [1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
            bubble_sort.aleo/sort([1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32])
        );
    }

    @test
    transition sort_3() {
        // input = [8, 7, 6, 5, 4, 3, 2, 1] (reverse order)
        assert_eq(
            [1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
            bubble_sort.aleo/sort([8u32, 7u32, 6u32, 5u32, 4u32, 3u32, 2u32, 1u32])
        );
    }

    @noupgrade
    async constructor() {}
}
