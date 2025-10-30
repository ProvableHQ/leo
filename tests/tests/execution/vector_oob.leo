program test_oob.aleo {
    storage vec: [u32];

    // OOB get: should return None
    async transition oob_get() -> Future {
        return async {
            vec.clear();
            assert(vec.len() == 0u32);

            let result = vec.get(0u32);
            assert(result == none);
        };
    }

    // OOB set: should panic (transition rejected)
    async transition oob_set() -> Future {
        return async {
            vec.clear();
            assert(vec.len() == 0u32);

            vec.set(0u32, 123u32);
        };
    }

    // OOB swap_remove: should panic (transition rejected)
    async transition oob_swap_remove() -> Future {
        return async {
            vec.clear();
            assert(vec.len() == 0u32);

            vec.swap_remove(0u32);
        };
    }

    // Pop empty vector: should return None
    async transition oob_pop() -> Future {
        return async {
            vec.clear();
            assert(vec.len() == 0u32);

            let popped = vec.pop();
            assert(popped == none);
        };
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test_oob.aleo;

program test_test_oob.aleo {

    // Safe: oob_get returns none
    @test
    async transition test_oob_get() -> Future {
        let f = test_oob.aleo/oob_get();
        return async {
            f.await();
        };
    }

    // Fails: oob_set is out-of-bounds
    @test
    @should_fail
    async transition test_oob_set() -> Future {
        let f = test_oob.aleo/oob_set();
        return async {
            f.await();
        };
    }

    // Fails: oob_swap_remove is out-of-bounds
    @test
    @should_fail
    async transition test_oob_swap_remove() -> Future {
        let f = test_oob.aleo/oob_swap_remove();
        return async {
            f.await();
        };
    }

    // Safe: oob_pop returns none
    @test
    async transition test_oob_pop() -> Future {
        let f = test_oob.aleo/oob_pop();
        return async {
            f.await();
        };
    }

    @noupgrade
    async constructor() {}
}
