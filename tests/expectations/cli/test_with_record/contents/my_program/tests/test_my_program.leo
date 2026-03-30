import my_program.aleo;

program test_my_program.aleo {
    // Verifies that double_amount computes x * 2.
    @test
    fn test_double_amount() {
        assert_eq(my_program.aleo::double_amount(5u64), 10u64);
        assert_eq(my_program.aleo::double_amount(0u64), 0u64);
    }

    @noupgrade
    constructor() {}
}
