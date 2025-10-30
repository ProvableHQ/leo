program test.aleo {
    struct S {
        x: u32,
        y: [u32; 3],
    }

    transition some_assignments(public a: bool, public b: bool) -> public [u32; 11] {
        let arr: [u32; 3] = [0u32, 1u32, 2u32];
        if a {
            arr[0u8] = 10u32;
        }

        let s: S = S {
            x: 3u32,
            y: arr,
        };

        let tuple: (S, S) = (s, s);

        if b {
            tuple.0.y[0u8] = 20u32;
            tuple.1.x = 30u32;
            if a {
                tuple.0 = s;
                tuple.1.y = [50u32, 60u32, 70u32];
            }
        }

        return [
            arr[0u8],
            arr[1u8],
            arr[2u8],
            tuple.0.x,
            tuple.0.y[0u8],
            tuple.0.y[1u8],
            tuple.0.y[2u8],
            tuple.1.x,
            tuple.1.y[0u8],
            tuple.1.y[1u8],
            tuple.1.y[2u8],
        ];
        
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;
program test_test.aleo {

    @test
    transition some_assignments_1() {
        // a=false, b=false
        assert_eq(
            [0u32, 1u32, 2u32, 3u32, 0u32, 1u32, 2u32, 3u32, 0u32, 1u32, 2u32],
            test.aleo/some_assignments(false, false)
        );
    }

    @test
    transition some_assignments_2() {
        // a=false, b=true
        assert_eq(
            [0u32, 1u32, 2u32, 3u32, 20u32, 1u32, 2u32, 30u32, 0u32, 1u32, 2u32],
            test.aleo/some_assignments(false, true)
        );
    }

    @test
    transition some_assignments_3() {
        // a=true, b=false
        assert_eq(
            [10u32, 1u32, 2u32, 3u32, 10u32, 1u32, 2u32, 3u32, 10u32, 1u32, 2u32],
            test.aleo/some_assignments(true, false)
        );
    }

    @test
    transition some_assignments_4() {
        // a=true, b=true
        assert_eq(
            [10u32, 1u32, 2u32, 3u32, 10u32, 1u32, 2u32, 30u32, 50u32, 60u32, 70u32],
            test.aleo/some_assignments(true, true)
        );
    }

    @noupgrade
    async constructor() {}
}

