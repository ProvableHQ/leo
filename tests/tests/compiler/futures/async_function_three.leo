
// Ensure Future types are correctly found through
// two layers of dependencies.

onchain fn finalize(x: u8) {
    assert_eq(1u8, 1u8);
}

program dependent.aleo {
    fn t1() -> Future {
        let f: Future = finalize(1u8);
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

onchain fn finalize(f: Future) {
    f.await();
}

program dependent2.aleo {
    fn t1() -> Future {
        let f: Future = dependent.aleo/t1();
        return finalize(f);
    }
}

// --- Next Program --- //

import dependent2.aleo;

onchain fn finalize(f: Future) {
    f.await();
}

program test.aleo {
    fn t1() -> Future {
        let f: Future = dependent2.aleo/t1();
        assert_eq(1u8, f.0.0);
        return finalize(f);
    }
}
