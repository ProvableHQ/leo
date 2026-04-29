// ANCHOR: file
// 4.0
import test_program.aleo;

program test_test_program.aleo {
    @test
    fn test_addition() {
        let result: u32 = test_program.aleo::sum(1u32, 2u32);
        assert_eq(result, 3u32);
    }

    @test
    @should_fail
    fn test_overflow() {
        let _result: u8 = test_program.aleo::sum_u8(255u8, 1u8);
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
