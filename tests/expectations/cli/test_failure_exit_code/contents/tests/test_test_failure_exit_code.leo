// A test that deliberately fails to verify non-zero exit code.

program test_test_failure_exit_code.aleo {
    @test
    fn test_deliberate_failure() {
        assert_eq(1u8, 2u8);
    }

    @noupgrade
    constructor() {}
}
