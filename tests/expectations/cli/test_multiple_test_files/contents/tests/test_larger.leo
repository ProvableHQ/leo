// The 'test_larger' test program.
import multi_test_prog.aleo;

program test_larger.aleo {
    @test
    fn adds_larger_numbers() {
        let result: u32 = multi_test_prog.aleo::main(10u32, 5u32);
        assert_eq(result, 15u32);
    }

    @test
    @should_fail
    fn wrong_expectation() {
        let result: u32 = multi_test_prog.aleo::main(2u32, 2u32);
        assert_eq(result, 5u32);
    }

    @noupgrade
    constructor() {}
}
