final fn finalize1(x: u8) {
    assert_eq(1u8, 1u8);
}

final fn finalize2(x: u16) {
    assert_eq(1u8, 1u8);
}

program dependent.aleo {
    fn t1() -> Final {
        let f: Final = final { finalize1(1u8); };
        return f;
    }
    fn t2() -> Final {
        let f: Final = final { finalize2(1u16); };
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

final fn finalize(f: Final) {
    f.run();
}

program test.aleo {
    fn t1() -> Final {
        let f: Final = dependent.aleo/t1();
        return final { finalize(f); };
    }

    fn t2() -> Final {
        let f: Final = dependent.aleo/t2();
        return final { finalize(f); };
    }
}
