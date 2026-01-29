program unsigned.aleo {
    storage counter_u8: u8;
    storage counter_u16: u16;
    storage counter_u32: u32;
    storage counter_u64: u64;
    storage counter_u128: u128;

    fn initialize() -> Final {
        return final {
            // Integers
            counter_u8 = 1;
            counter_u16 = 2;
            counter_u32 = 3;
            counter_u64 = 4;
            counter_u128 = 5;
        };
    }

    fn to_none() -> Final {
        return final {
            // Integers
            counter_u8 = none;
            counter_u16 = none;
            counter_u32 = none;
            counter_u64 = none;
            counter_u128 = none;
        };
    }

    fn check1() -> Final {
        return final {
            // Integers
            assert(counter_u8.unwrap() == 1);
            assert(counter_u16.unwrap() == 2);
            assert(counter_u32.unwrap() == 3);
            assert(counter_u64.unwrap() == 4);
            assert(counter_u128.unwrap() == 5);
        };
    }

    fn check2() -> Final {
        return final {
            // Integers
            assert(counter_u8.unwrap_or(0) == 1);
            assert(counter_u16.unwrap_or(0) == 2);
            assert(counter_u32.unwrap_or(0) == 3);
            assert(counter_u64.unwrap_or(0) == 4);
            assert(counter_u128.unwrap_or(0) == 5);
        };
    }

    @noupgrade
    constructor() {}
}
