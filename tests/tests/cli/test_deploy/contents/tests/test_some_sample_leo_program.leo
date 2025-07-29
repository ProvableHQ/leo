// The 'test_test_deploy_leo_program' test program.
import test_deploy_leo_program.aleo;
program test_test_deploy_leo_program.aleo {
    @test
    script test_it() {
        let result: u32 = test_deploy_leo_program.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }

    @test
    @should_fail
    transition do_nothing() {
        let result: u32 = test_deploy_leo_program.aleo/main(2u32, 3u32);
        assert_eq(result, 3u32);
    }
}
