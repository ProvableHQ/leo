program test.aleo {
    transition main(a: group) -> (field, field) {
        let b: field = group::to_x_coordinate(a);
        let c: field = a as field;
        let e: field = a.to_x_coordinate();
        assert_eq(b, c);
        assert_eq(b, e);
        let d: field = group::to_y_coordinate(a);
        let f: field = a.to_y_coordinate();
        assert_eq(d, f);
        return (b, d);
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;

program test_test.aleo {

    @test
    transition main_0group() {
        let (x, y) = test.aleo/main(0group);
        // `main` already asserts internally, so we just call it
        assert(x == x); // Dummy assertion to use `x` and `y`
        assert(y == y);
    }

    @test
    transition main_2group() {
        let (x, y) = test.aleo/main(2group);
        assert(x == x);
        assert(y == y);
    }

    @noupgrade
    async constructor() {}
}
