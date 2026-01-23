onchain fn finalize1(x: u8) {
    assert_eq(1u8, 1u8);
}

onchain fn finalize2(x: u16) {
    assert_eq(1u8, 1u8);
}

program dependent.aleo {
    fn t1() -> Final {
        let f: Final = finalize1(1u8);
        return f;
    }
    fn t2() -> Final {
        let f: Final = finalize2(1u16);
        return f;
    }
}

// --- Next Program --- //

import dependent.aleo;

onchain fn finalize(f: Final) {
    let x: u8 = f.0;
    f.await();
}

program test.aleo {
    fn t1() -> Final {
        let f: Final = dependent.aleo/t1();
        return finalize(f);
    }

    fn t2() -> Final {
        let f: Final = dependent.aleo/t2();
        return finalize(f);
    }
}
