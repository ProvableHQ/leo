program compare_optionals.aleo {
    struct Point {
        x: u8,
        y: u8,
    }

    // u8? == none → false
    transition u8_some_eq_none() -> bool {
        let val: u8? = 5u8;
        return val == none;
    }

    // i128? == none → true (comparing two none values)
    transition i128_none_eq_none() -> bool {
        let a: i128? = none;
        let b: i128? = none;
        return a == b;
    }

    // u8? == u8? (same value) → true
    transition u8_some_eq_some() -> bool {
        let a: u8? = 42u8;
        let b: u8? = 42u8;
        return a == b;
    }

    // i128? != i128? (different values) → true
    transition i128_some_neq_some() -> bool {
        let a: i128? = 100i128;
        let b: i128? = -50i128;
        return a != b;
    }

    // struct? == struct? (same fields) → true
    transition struct_some_eq_some() -> bool {
        let a: Point? = Point { x: 1u8, y: 2u8 };
        let b: Point? = Point { x: 1u8, y: 2u8 };
        return a == b;
    }

    // struct? != none → true
    transition struct_some_neq_none() -> bool {
        let a: Point? = Point { x: 5u8, y: 9u8 };
        return a != none;
    }

    // [u8; 2]? == [u8; 2]? → true
    transition array_some_eq_some() -> bool {
        let a: [u8; 2]? = [7u8, 8u8];
        let b: [u8; 2]? = [7u8, 8u8];
        return a == b;
    }

    // [u8; 2]? != [u8; 2]? → true
    transition array_some_neq_some() -> bool {
        let a: [u8; 2]? = [1u8, 2u8];
        let b: [u8; 2]? = [3u8, 4u8];
        return a != b;
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import compare_optionals.aleo;
program test_compare_optionals.aleo {

    @test
    transition u8_some_eq_none_test() {
        assert_eq(false, compare_optionals.aleo/u8_some_eq_none());
    }

    @test
    transition i128_none_eq_none_test() {
        assert_eq(true, compare_optionals.aleo/i128_none_eq_none());
    }

    @test
    transition u8_some_eq_some_test() {
        assert_eq(true, compare_optionals.aleo/u8_some_eq_some());
    }

    @test
    transition i128_some_neq_some_test() {
        assert_eq(true, compare_optionals.aleo/i128_some_neq_some());
    }

    @test
    transition struct_some_eq_some_test() {
        assert_eq(true, compare_optionals.aleo/struct_some_eq_some());
    }

    @test
    transition struct_some_neq_none_test() {
        assert_eq(true, compare_optionals.aleo/struct_some_neq_none());
    }

    @test
    transition array_some_eq_some_test() {
        assert_eq(true, compare_optionals.aleo/array_some_eq_some());
    }

    @test
    transition array_some_neq_some_test() {
        assert_eq(true, compare_optionals.aleo/array_some_neq_some());
    }

    @noupgrade
    async constructor() {}
}
