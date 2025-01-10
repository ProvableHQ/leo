/*
namespace = "Compile"
expectation = "Pass"
*/
program dependent.aleo {
    async function finalize1(x: u8) {
        assert_eq(1u8, 1u8);
    }

    async transition t1() -> Future {
        let f: Future = finalize1(1u8);
        return f;
    }

    async function finalize2(x: u8) {
        assert_eq(1u8, 1u8);
    }

    async transition t2() -> Future {
        let f: Future = finalize2(1u8);
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

program test.aleo {
    async function finalize(f: Future) {
        let x: u8 = f.0;
        f.await();
    }

    async transition t1() -> Future {
        let f: Future = dependent.aleo/t1();
        return finalize(f);
    }

    async transition t2() -> Future {
        let f: Future = dependent.aleo/t2();
        return finalize(f);
    }
}
