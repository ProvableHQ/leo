program test.aleo {
    transition main() {
        // Overflow examples for unsigned integers
        let a0: u8 = 256;
        let a1 = 1u8 + 256;

        let a2: u16 = 65_536;
        let a3 = 1u16 + 65_536;

        let a4: u32 = 4_294_967_296;
        let a5 = 1u32 + 4_294_967_296;

        let a6: u64 = 18_446_744_073_709_551_616;
        let a7 = 1u64 + 18_446_744_073_709_551_616;

        let a8: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_456;
        let a9 = 1u128 + 340_282_366_920_938_463_463_374_607_431_768_211_456;

        // Overflow examples for signed integers
        let a10: i8 = 128;
        let a11 = 1i8 + 128;

        let a12: i16 = 32_768;
        let a13 = 1i16 + 32_768;

        let a14: i32 = 2_147_483_648;
        let a15 = 1i32 + 2_147_483_648;

        let a16: i64 = 9_223_372_036_854_775_808;
        let a17 = 1i64 + 9_223_372_036_854_775_808;

        let a18: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_728;
        let a19 = 1i128 + 170_141_183_460_469_231_731_687_303_715_884_105_728;

        // 1 is not a valid group
        let a20: group = 1;
        let a21 = 0group + 1;

        // Overflow in loop bounds
        for i: u8 in 1..256 {}
    }
}
