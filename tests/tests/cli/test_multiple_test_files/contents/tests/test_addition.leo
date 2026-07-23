// The 'test_addition' test program.
import multi_test_prog.aleo;

program test_addition.aleo {
    @test
    fn adds_small_numbers() {
        let result: u32 = multi_test_prog.aleo::main(1u32, 2u32);
        assert_eq(result, 3u32);
    }

    @noupgrade
    constructor() {}
}
