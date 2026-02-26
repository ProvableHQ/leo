// The 'test_some_sample_leo_program' test program.
import some_sample_leo_program.aleo;

program test_some_sample_leo_program.aleo {
    @test
    fn test_it() {
        let result: u32 = some_sample_leo_program.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }

    @test
    @should_fail
    fn do_nothing() {
        let result: u32 = some_sample_leo_program.aleo/main(2u32, 3u32);
        assert_eq(result, 3u32);
    }
}
