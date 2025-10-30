program test.aleo {
    transition main(x: u32, y: u32) -> bool {
        assert_eq(x, y);
        return x == y;
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;

program test_test.aleo {

    @test
    @should_fail
    transition main_0_1() {
        assert_eq(false, test.aleo/main(0u32, 1u32));
    }

    @test
    transition main_1_1() {
        assert_eq(true, test.aleo/main(1u32, 1u32));
    }

    @test
    @should_fail
    transition main_1_2() {
        assert_eq(false, test.aleo/main(1u32, 2u32));
    }

    @test
    transition main_2_2() {
        assert_eq(true, test.aleo/main(2u32, 2u32));
    }

    @noupgrade
    async constructor() {}
}
