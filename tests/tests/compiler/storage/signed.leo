program signed.aleo {
    storage counter_i8: i8;
    storage counter_i16: i16;
    storage counter_i32: i32;
    storage counter_i64: i64;
    storage counter_i128: i128;

    async transition initialize() -> Future {
        return async {
            counter_i8 = -1i8;
            counter_i16 = -2i16;
            counter_i32 = -3i32;
            counter_i64 = -4i64;
            counter_i128 = -5i128;
        };
    }

    async transition to_none() -> Future {
        return async {
            // Integers
            counter_i8 = none;
            counter_i16 = none;
            counter_i32 = none;
            counter_i64 = none;
            counter_i128 = none;
        };
    }

    async transition check1() -> Future {
        return async {
            assert(counter_i8.unwrap() == -1i8);
            assert(counter_i16.unwrap() == -2i16);
            assert(counter_i32.unwrap() == -3i32);
            assert(counter_i64.unwrap() == -4i64);
            assert(counter_i128.unwrap() == -5i128);
        };
    }

    async transition check2() -> Future {
        return async {
            assert(counter_i8.unwrap_or(0) == -1i8);
            assert(counter_i16.unwrap_or(0) == -2i16);
            assert(counter_i32.unwrap_or(0) == -3i32);
            assert(counter_i64.unwrap_or(0) == -4i64);
            assert(counter_i128.unwrap_or(0) == -5i128);
        };
    }

    @noupgrade
    async constructor() {}
}
