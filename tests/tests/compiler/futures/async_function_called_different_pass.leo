program dependent.aleo {
    fn t1() -> Final {
        let f: Final = final { finalize1(1u8); };
        return f;
    }
    fn t2() -> Final {
        let f: Final = final { finalize2(1u8); };
        return f;
    }
}

final fn finalize1(x: u8) {
    assert_eq(1u8, 1u8);
}
final fn finalize2(x: u8) {
    assert_eq(1u8, 1u8);
}

// --- Next Program --- //

import dependent.aleo;

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

final fn finalize(f: Final) {
    f.run();
}

