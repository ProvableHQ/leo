program test.aleo {
    transition some_assignments(public a: bool, public b: bool, public c: bool) -> public [[u32; 2]; 2] {
        let arr: [[u32; 2]; 2] = [[1u32, 2u32], [3u32, 4u32]];
        if a {
            arr[0u8][0u8] = 10u32;
            arr[0u8][1u8] = 20u32;
        } else {
            arr[1u8][0u8] = 30u32;
        }
        if b {
            arr[0u8] = [100u32, 200u32];
        }
        if c {
            arr[0u8][0u8] = 1000u32;
            arr[0u8][1u8] = 2000u32;
        }
        return arr;
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;
program test_test.aleo {

    @test
    transition some_assignments_1() {
        // a=false, b=false, c=false
        assert_eq([[1u32, 2u32], [30u32, 4u32]], test.aleo/some_assignments(false, false, false));
    }

    @test
    transition some_assignments_2() {
        // a=false, b=false, c=true
        assert_eq([[1000u32, 2000u32], [30u32, 4u32]], test.aleo/some_assignments(false, false, true));
    }

    @test
    transition some_assignments_3() {
        // a=false, b=true, c=false
        assert_eq([[100u32, 200u32], [30u32, 4u32]], test.aleo/some_assignments(false, true, false));
    }

    @test
    transition some_assignments_4() {
        // a=false, b=true, c=true
        assert_eq([[1000u32, 2000u32], [30u32, 4u32]], test.aleo/some_assignments(false, true, true));
    }

    @test
    transition some_assignments_5() {
        // a=true, b=false, c=false
        assert_eq([[10u32, 20u32], [3u32, 4u32]], test.aleo/some_assignments(true, false, false));
    }

    @test
    transition some_assignments_6() {
        // a=true, b=false, c=true
        assert_eq([[1000u32, 2000u32], [3u32, 4u32]], test.aleo/some_assignments(true, false, true));
    }

    @test
    transition some_assignments_7() {
        // a=true, b=true, c=false
        assert_eq([[100u32, 200u32], [3u32, 4u32]], test.aleo/some_assignments(true, true, false));
    }

    @test
    transition some_assignments_8() {
        // a=true, b=true, c=true
        assert_eq([[1000u32, 2000u32], [3u32, 4u32]], test.aleo/some_assignments(true, true, true));
    }

    @noupgrade
    async constructor() {}
}

