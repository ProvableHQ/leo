// The 'test_test_simple_build_program' test program.
import test_simple_build_program.aleo;
program test_test_simple_build_program.aleo {
    @test
    script test_it() {
        let result: u32 = test_simple_build_program.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }

    @test
    @should_fail
    transition do_nothing() {
        let result: u32 = test_simple_build_program.aleo/main(2u32, 3u32);
        assert_eq(result, 3u32);
    }
}
