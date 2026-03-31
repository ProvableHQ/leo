program test_my_lib.aleo {
    @test
    fn test_double() {
        assert_eq(my_lib::double(5u32), 10u32);
    }

    @test
    fn test_square() {
        assert_eq(my_lib::square(4u32), 16u32);
    }

    @test
    fn test_square_zero() {
        assert_eq(my_lib::square(0u32), 0u32);
    }

    @noupgrade
    constructor() {}
}
