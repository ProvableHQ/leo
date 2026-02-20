// The 'test_test_program2' test program.
import test_program2.aleo;
program test_test_program2.aleo {
    @test
    @should_fail
    fn do_nothing() {
        let result: u32 = test_program2.aleo/main(2u32, 3u32);
        assert_eq(result, 3u32);
    }
}

@test
script test_it() {
    let result: u32 = test_program2.aleo/main(1u32, 2u32);
    assert_eq(result, 3u32);
}
