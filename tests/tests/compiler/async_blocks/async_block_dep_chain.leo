
// Ensure Future types are correctly found through
// two layers of dependencies.

program dependent.aleo {
    async transition t1() -> Future {
        let x = 1u8;
        let y = 2u16;
        let f: Future = async {
            assert_eq(x, 1u8);
            assert_eq(y, 2u16);
        };
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

program dependent2.aleo {
    async transition t1() -> Future {
        let f: Future = dependent.aleo/t1();
        return async { f.await(); };
    }
}

// --- Next Program --- //

import dependent2.aleo;

program test.aleo {
    async transition t1() -> Future {
        let f: Future = dependent2.aleo/t1();
        assert_eq(1u8, f.0.0);
        assert_eq(1u16, f.0.1);
        return async { f.await(); };
    }
}
