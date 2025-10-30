// Note the input height is 23, since the test framework creates a few more blocks to set up the VM.

program test.aleo {
    async transition check_starting_height(height: u32) -> Future {
        return check_starting_height_(height);
    }

    async function check_starting_height_(height: u32) {
        assert(height <= block.height);
    }

    @noupgrade
    async constructor() {}
}

// --- DO NOT RUN THIS TEST JUST YET --- //

import test.aleo;
program test_test.aleo {

    @test
    async transition check_starting_height() -> Future {
        let f = test.aleo/check_starting_height(23u32);
        return async {
            f.await();
        };
    }

    @noupgrade
    async constructor() {}
}
