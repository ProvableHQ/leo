program test.aleo {
    storage counter: u32;

    async transition initialize() -> Future {
        return async {
            counter = 0;
        };
    }

    async transition increment() -> Future {
        return async {
            let old_counter_value = counter.unwrap_or(0);
            counter = old_counter_value + 1u32;
            assert(counter.unwrap() == old_counter_value + 1u32);
        };
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;

program test_test.aleo {

    @test
    async transition increment_first() -> Future {
        let f = test.aleo/increment();
        return async {
            f.await();
        };
    }

    @test
    async transition increment_second() -> Future {
        let f = test.aleo/increment();
        return async {
            f.await();
        };
    }

    @test
    async transition increment_third() -> Future {
        let f = test.aleo/increment();
        return async {
            f.await();
        };
    }

    @test
    async transition increment_fourth() -> Future {
        let f = test.aleo/increment();
        return async {
            f.await();
        };
    }

    @noupgrade
    async constructor() {}
}
