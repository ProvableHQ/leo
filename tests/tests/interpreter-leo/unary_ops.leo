program test.aleo {
    transition main() {
        // Abs
        assert_eq((-1i8).abs(), 1i8);
        assert_eq((-1i16).abs(), 1i16);
        assert_eq((-1i32).abs(), 1i32);
        assert_eq((-1i64).abs(), 1i64);
        assert_eq((-1i128).abs(), 1i128);

        // Abs wrapped for MIN values
        assert_eq((-128i8).abs_wrapped(), -128i8);     // i8::MIN
        assert_eq((-32_768i16).abs_wrapped(), -32_768i16); // i16::MIN
        assert_eq((-2_147_483_648i32).abs_wrapped(), -2_147_483_648i32); // i32::MIN
        assert_eq((-9_223_372_036_854_775_808i64).abs_wrapped(), -9_223_372_036_854_775_808i64); // i64::MIN
        assert_eq((-170_141_183_460_469_231_731_687_303_715_884_105_728i128).abs_wrapped(), 
                   -170_141_183_460_469_231_731_687_303_715_884_105_728i128); // i128::MIN
   
        // Double
        assert_eq(2group.double(), 6696402423798020098358712667671415812305707015226794708266486692814448135893group);
        assert_eq(2field.double(), 4field);

        // Inverse
        assert_eq(10field.inv(), 7600015574485533381823942444903391878238309401638657445141710110325668315137field);

        // Neg
        assert_eq(10field.neg(), 8444461749428370424248824938781546531375899335154063827935233455917409239031field);
        assert_eq(2group.neg(), 8444461749428370424248824938781546531375899335154063827935233455917409239039group);
        assert_eq((-1i8).neg(), 1i8);
        assert_eq((-1i16).neg(), 1i16);
        assert_eq((-1i32).neg(), 1i32);
        assert_eq((-1i64).neg(), 1i64);
        assert_eq((-1i128).neg(), 1i128);

        // Not
        assert_eq((true).not(), false);
        assert_eq((false).not(), true);
        assert_eq((42i8).not(), -43i8);                         // 0b00101010 -> 0b11010101
        assert_eq((-1000i16).not(), 999i16);
        assert_eq((123456i32).not(), -123457i32);
        assert_eq((-9876543210i64).not(), 9876543209i64);
        assert_eq((314159265358979i128).not(), -314159265358980i128);
        assert_eq((200u8).not(), 55u8);                         // 0b11001000 -> 0b00110111
        assert_eq((50000u16).not(), 15535u16);
        assert_eq((1000000000u32).not(), 3294967295u32);
        assert_eq((9876543210987654321u64).not(), 8570200862721897294u64);
        assert_eq(
            (170141183460469231731687303715884105727u128).not(),
             170141183460469231731687303715884105728u128
        );

        // Square
        assert_eq(50field.square(), 2500field);

        // Square Root
        assert_eq(2500field.square_root(), 50field);

        // ToXCoordinate
        assert_eq(2group.to_x_coordinate(), 2field);

        // ToYCoordinate
        assert_eq(2group.to_y_coordinate(), 5553594316923449299484601589326170487897520766531075014687114064346375156608field);
    }
}
