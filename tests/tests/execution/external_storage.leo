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

    fn c() -> Final {
        let f = final {
            count = count.unwrap_or(0) + 1u64;
        };
        return f;
    }

    @noupgrade
    constructor() {}
}


// --- Next Program --- //


program one_program.aleo {
    storage count: u64;

    fn d() -> Final {
        let f = final {
            count = count.unwrap_or(0) + 2u64;
        };
        return f;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;

program two_program.aleo {
    storage count: u64;

    fn b() -> Final {
        let f0: Final = zero_program.aleo/c();
        let f1: Final = one_program.aleo/d();

        return final {
            f0.run();
            f1.run();
            assert(zero_program.aleo/count.unwrap() == 1u64);
            assert(one_program.aleo/count.unwrap() == 2u64);
            count = zero_program.aleo/count.unwrap() + one_program.aleo/count.unwrap();
            assert(count.unwrap() == 3u64);
        };
    }

    @noupgrade
    constructor() {}
}
