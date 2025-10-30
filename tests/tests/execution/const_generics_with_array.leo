program test.aleo {
    inline foo::[M: u32](arr: [u32; 3]) -> u32 {
        return arr[M - 1];
    }

    transition main() -> u32 {
        return foo::[3u32]([1, 2, 3]);
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;
program test_test.aleo {

    @test
    transition main_1() {
        assert_eq(3u32, test.aleo/main());
    }

    @noupgrade
    async constructor() {}
}
