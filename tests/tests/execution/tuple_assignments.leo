program test.aleo {
    transition main(a: u64, b: u64, c: u64, d: u64, y: bool, w: bool, z: bool) -> (public u64, public u64, public u64, public u64) {
        let tuple: (u64, u64, u64, u64) = (a, b, c, d);

        if y {
            if w {
                tuple.0 = 100u64;
                tuple.1 = 200u64;
            } else {
                tuple.2 = 300u64;
            }
            tuple.3 = 400u64;
        } else if z {
            if w {
                tuple.0 = 500u64;
                tuple.2 = 600u64;
            } else {
                tuple.1 = 700u64;
            }
        } else {
            tuple.3 = 800u64;
        }

        return tuple;
    }

      @noupgrade
      async constructor() {}
}

// --- Test --- //

import test.aleo;
program test_test.aleo {

    @test
    transition main_case1() {
        let (a, b, c, d) = test.aleo/main(1u64, 2u64, 3u64, 4u64, true, true, true);
        assert_eq(a, 100u64);
        assert_eq(b, 200u64);
        assert_eq(c, 3u64);
        assert_eq(d, 400u64);
    }

    @test
    transition main_case2() {
        let (a, b, c, d) = test.aleo/main(1u64, 2u64, 3u64, 4u64, false, true, true);
        assert_eq(a, 500u64);
        assert_eq(b, 2u64);
        assert_eq(c, 600u64);
        assert_eq(d, 4u64);
    }

    @test
    transition main_case3() {
        let (a, b, c, d) = test.aleo/main(1u64, 2u64, 3u64, 4u64, false, true, false);
        assert_eq(a, 1u64);
        assert_eq(b, 2u64);
        assert_eq(c, 3u64);
        assert_eq(d, 800u64);
    }

    @test
    transition main_case4() {
        let (a, b, c, d) = test.aleo/main(1u64, 2u64, 3u64, 4u64, false, false, true);
        assert_eq(a, 1u64);
        assert_eq(b, 700u64);
        assert_eq(c, 3u64);
        assert_eq(d, 4u64);
    }

    @test
    transition main_case5() {
        let (a, b, c, d) = test.aleo/main(1u64, 2u64, 3u64, 4u64, false, false, false);
        assert_eq(a, 1u64);
        assert_eq(b, 2u64);
        assert_eq(c, 3u64);
        assert_eq(d, 800u64);
    }

    @noupgrade
    async constructor() {}
}

