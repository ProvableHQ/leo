import example_program.aleo;

program test_example_program.aleo {
    // ANCHOR: test_simple_addition
    @test
    fn test_simple_addition() {
        let result: u32 = example_program.aleo::simple_addition(2u32, 3u32);
        assert_eq(result, 5u32);
    }
    // ANCHOR_END: test_simple_addition

    // ANCHOR: test_simple_addition_fail
    @test
    @should_fail
    fn test_simple_addition_fail() {
        let result: u32 = example_program.aleo::simple_addition(2u32, 3u32);
        assert_eq(result, 3u32);
    }
    // ANCHOR_END: test_simple_addition_fail

    // ANCHOR: test_record_maker
    @test
    fn test_record_maker() {
        let r: example_program.aleo::Example = example_program.aleo::mint_record(0field);
        assert_eq(r.x, 0field);
    }
    // ANCHOR_END: test_record_maker

    // ANCHOR: test_with_private_key
    // Run this test as a specific account, not the default test account.
    // `std::ctx::caller()` and `std::ctx::signer()` resolve to the address derived from
    // the supplied private key for the duration of this test.
    @test(private_key = "APrivateKey1zkpG9Af9z5Ha4ejVyMCqVFXRKknSm8L1ELEwcc4htk9YhVK")
    fn test_as_specific_account() {
        let result: u32 = example_program.aleo::simple_addition(2u32, 3u32);
        assert_eq(result, 5u32);
    }
    // ANCHOR_END: test_with_private_key

    // ANCHOR: test_admin_pair
    @test(private_key = "APrivateKey1zkpG9Af9z5Ha4ejVyMCqVFXRKknSm8L1ELEwcc4htk9YhVK")
    fn test_admin_can_pause() {
        example_program.aleo::pause();
    }

    @test
    @should_fail
    fn test_non_admin_cannot_pause() {
        example_program.aleo::pause();
    }
    // ANCHOR_END: test_admin_pair

    @noupgrade
    constructor() {}
}
