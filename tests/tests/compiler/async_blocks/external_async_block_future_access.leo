program dependent.aleo {
    fn t1(x: u8, y: u16) -> Final {
        return final {
            assert_eq(x, 1u8);
            assert_eq(y, 1u16);
        };
    }
}

// --- Next Program --- //

import dependent.aleo;

program test.aleo {
    fn t1() -> Final {
        let f: Final = dependent.aleo/t1(42, 43);
        return final {
            f.run();
        };
    }
}
