
// Ensure Final types are correctly found through
// two layers of dependencies.

program dependent.aleo {
    fn t1() -> Final {
        let x = 1u8;
        let y = 2u16;
        let f: Final = final {
            assert_eq(x, 1u8);
            assert_eq(y, 2u16);
        };
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

program dependent2.aleo {
    fn t1() -> Final {
        let f: Final = dependent.aleo/t1();
        return final { f.run(); };
    }
}

// --- Next Program --- //

import dependent2.aleo;

program test.aleo {
    fn t1() -> Final {
        let f: Final = dependent2.aleo/t1();
        return final { f.run(); };
    }
}
