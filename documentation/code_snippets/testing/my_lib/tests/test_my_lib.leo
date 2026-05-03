// ANCHOR: test_program
program test_my_lib.aleo {
    @test
    fn test_double() {
        assert_eq(my_lib::double(5u32), 10u32);
    }

    @test
    fn test_triple() {
        assert_eq(my_lib::math::triple(4u32), 12u32);
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: test_program
