program test_failure_exit_code.aleo {
    @test
    transition test_should_fail() {
        assert_eq(1u8, 2u8);
    }
}
