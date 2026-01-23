program dependent.aleo {
    fn t1(x: u8, y: u16) -> Final {
        return async {
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
        return async {
            let x: u16 = f.0;
            let y: u8 = f.1;
            let z = f.2;
            f.await();           
        };
    }
}
