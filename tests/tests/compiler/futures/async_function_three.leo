/*
namespace = "Compile"
expectation = "Pass"
*/

// Ensure Future types are correctly found through
// two layers of dependencies.

program dependent.aleo {
    async function finalize(x: u8) {
        assert_eq(1u8, 1u8);
    }

    async transition t1() -> Future {
        let f: Future = finalize(1u8);
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

program dependent2.aleo {
    async function finalize(f: Future) {
        f.await();
    }

    async transition t1() -> Future {
        let f: Future = dependent.aleo/t1();
        return finalize(f);
    }
}

// --- Next Program --- //

import dependent2.aleo;

program test.aleo {
    async function finalize(f: Future) {
        f.await();
    }

    async transition t1() -> Future {
        let f: Future = dependent2.aleo/t1();
        assert_eq(1u8, f.0.0);
        return finalize(f);
    }
}
