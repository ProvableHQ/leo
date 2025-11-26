/*
seed = 123456789
min_height = 16

[case]
program = "two_program.aleo"
function = "b"
input = []
*/

program zero_program.aleo {
    storage count: u64;

    async transition c() -> Future {
        let f = async {
            count = count.unwrap_or(0) + 1u64;
        };
        return f;
    }

    @noupgrade
    async constructor() {}
}


// --- Next Program --- //


program one_program.aleo {
    storage count: u64;

    async transition d() -> Future {
        let f = async {
            count = count.unwrap_or(0) + 2u64;
        };
        return f;
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;

program two_program.aleo {
    storage count: u64;

    async transition b() -> Future {
        let f0: Future = zero_program.aleo/c();
        let f1: Future = one_program.aleo/d();

        return async {
            f0.await();
            f1.await();
            assert(zero_program.aleo/count.unwrap() == 1u64);
            assert(one_program.aleo/count.unwrap() == 2u64);
            count = zero_program.aleo/count.unwrap() + one_program.aleo/count.unwrap();
            assert(count.unwrap() == 3u64);
        };
    }

    @noupgrade
    async constructor() {}
}
