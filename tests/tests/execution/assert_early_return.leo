// The intent is to test that the assertion
// does not trigger if the early return happens.
program test.aleo {
    transition main(x: u32) -> bool {
        if x < 5u32 {
            return true;
        }
        assert(false);
        return true;
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;
program test_test.aleo {

    @test
    transition main_1() {
        // x = 1u32
        assert_eq(true, test.aleo/main(1u32));
    }

    @test
    transition main_2() {
        // x = 2u32
        assert_eq(true, test.aleo/main(2u32));
    }

    @test
    transition main_3() {
        // x = 3u32
        assert_eq(true, test.aleo/main(3u32));
    }

    @test
    transition main_4() {
        // x = 4u32
        assert_eq(true, test.aleo/main(4u32));
    }

    @test
    @should_fail
    transition main_5() {
        // x = 5u32
        assert_eq(true, test.aleo/main(5u32));
    }

    @noupgrade
    async constructor() {}
}
