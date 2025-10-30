// The 'some_sample_leo_program' program.
program simple.aleo {
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

// The 'test_some_sample_leo_program' test program.
import simple.aleo;
program test_simple.aleo {
    @test
    script test_it() {
        let result: u32 = simple.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }

    @test
    @should_fail
    transition do_nothing() {
        let result: u32 = simple.aleo/main(2u32, 3u32);
        assert_eq(result, 3u32);
    }

    @noupgrade
    async constructor() {}
}
