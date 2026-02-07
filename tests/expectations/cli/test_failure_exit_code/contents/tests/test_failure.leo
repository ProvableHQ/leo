// Test to verify that test failures exit with non-zero exit code
import test_failure_exit_code.aleo;

program test_failure.aleo {
    @test
    transition test_should_fail() {
        assert_eq(1u8, 2u8);
    }
}
