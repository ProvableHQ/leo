
// Ensure Final types are correctly found through
// two layers of dependencies.

final fn finalize(x: u8) {
    assert_eq(1u8, 1u8);
}

program dependent.aleo {
    fn t1() -> Final {
        let f: Final = final { finalize(1u8); };
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

final fn finalize(f: Final) {
    f.run();
}

program dependent2.aleo {
    fn t1() -> Final {
        let f: Final = dependent.aleo/t1();
        return final { finalize(f); };
    }
}

// --- Next Program --- //

import dependent2.aleo;

final fn finalize(f: Final) {
    f.run();
}

program test.aleo {
    fn t1() -> Final {
        let f: Final = dependent2.aleo/t1();
        return final { finalize(f); };
    }
}
