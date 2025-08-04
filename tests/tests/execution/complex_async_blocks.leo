/*
seed = 123456789
min_height = 16

[case]
program = "four_program.aleo"
function = "a"
input = []
*/

// This test checks that the finalization order is correct.
// The functions are invoked in the following order:
// "four::a"
//   --> "two::b"
//        --> "zero::c"
//        --> "one::d"
//   --> "three::e"
//        --> "two::b"
//             --> "zero::c"
//             --> "one::d"
//        --> "one::d"
//        --> "zero::c"
// The future (call graph) produced by the top-level finalize should reflect this structure.

program zero_program.aleo {
    mapping counts: address => u64;

    async transition c() -> Future {
        let addr = self.signer;
        let f = async {
            let count: u64 = counts.get_or_use(addr, 0u64);
            counts.set(addr, count + 1u64);
        };
        return f;
    }

    @noupgrade
    async constructor() {}
}


// --- Next Program --- //


program one_program.aleo {
    mapping counts: address => u64;

    async transition d() -> Future {
        let addr = self.signer;
        let f = async {
            let count: u64 = counts.get_or_use(addr, 0u64);
            counts.set(addr, count + 1u64);
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
    mapping counts: address => u64;

    async transition b() -> Future {
        let f0: Future = zero_program.aleo/c();
        let f1: Future = one_program.aleo/d();
        let addr = self.signer; 

        return async {
            f0.await();
            f1.await();
            let count: u64 = counts.get_or_use(addr, 0u64);
            counts.set(addr, count + 1u64);
        };
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import zero_program.aleo;
import one_program.aleo;
import two_program.aleo;

program three_program.aleo {
    mapping counts: address => u64;

    async transition e() -> Future {
        let f0: Future = two_program.aleo/b();
        let f1: Future = one_program.aleo/d();
        let f2: Future = zero_program.aleo/c();
        let addr = self.signer;
        return async {
            f0.await();
            f1.await();
            f2.await();
            let count: u64 = counts.get_or_use(addr, 0u64);
            counts.set(addr, count + 1u64);
        };
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import two_program.aleo;
import three_program.aleo;

program four_program.aleo {
    mapping counts: address => u64;

    async transition a() -> Future {
        let f0: Future = two_program.aleo/b();
        let f1: Future = three_program.aleo/e();
        let addr = self.signer;
        return async {
            f0.await();
            f1.await();
            let count: u64 = counts.get_or_use(addr, 0u64);
            counts.set(addr, count + 1u64);
        };
    }

    @noupgrade
    async constructor() {}
}
