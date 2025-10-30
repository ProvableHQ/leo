program test.aleo {
    transition main(x: u32) -> bool {
        let c: u32 = 0u32;

        if x == 1u32 {
            c = 1u32;
        } else if x == 2u32 {
            c = 2u32;
        } else {
            c = 3u32;
        }
        return c == x;
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
        assert_eq(false, test.aleo/main(4u32));
    }

    @noupgrade
    async constructor() {}
}
