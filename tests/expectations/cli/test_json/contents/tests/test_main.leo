import test_json_program.aleo;
program test_main.aleo {
    // Required by snarkVM - every program needs at least one transition.
    transition placeholder() {}

    @test
    script test_addition() {
        let result: u32 = test_json_program.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }
}
