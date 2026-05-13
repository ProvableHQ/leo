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

    @noupgrade
    constructor() {}
}
