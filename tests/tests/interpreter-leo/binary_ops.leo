program test.aleo {
    struct Point {
        x: u32,
        y: u32,
    }

    struct Point2 {
        x: u32,
        y: u32,
    }

    transition main() {
        // Add
        assert_eq(1i8 + 2i8, 3i8);
        assert_eq(1000i16 + 2345i16, 3345i16);
        assert_eq(123456i32 + 654321i32, 777777i32);
        assert_eq(9876543210i64 + 1234567890i64, 11111111100i64);
        assert_eq(314159265358979i128 + 271828182845904i128, 585987448204883i128);
        assert_eq(200u8 + 55u8, 255u8);
        assert_eq(50000u16 + 15535u16, 65535u16);
        assert_eq(1000000000u32 + 3000000000u32, 4000000000u32);
        assert_eq(9876543210987654321u64 + 1234567890123456789u64, 11111111101111111110u64);
        assert_eq(170141183460469231731687303715884105727u128 + 1u128, 170141183460469231731687303715884105728u128);
        assert_eq(2group + 2group, 6696402423798020098358712667671415812305707015226794708266486692814448135893group);
        assert_eq(3field + 5field, 8field);
        assert_eq(5scalar + 5scalar, 10scalar);

        // Add Wrapped
        assert_eq(120i8.add_wrapped(10i8), -126i8);
        assert_eq(32000i16.add_wrapped(1000i16), -32536i16);
        assert_eq(2_147_483_640i32.add_wrapped(20i32), -2_147_483_636i32);
        assert_eq(9_223_372_036_854_775_800i64.add_wrapped(20i64), -9_223_372_036_854_775_796i64);
        assert_eq(170141183460469231731687303715884105720i128.add_wrapped(10i128), -170141183460469231731687303715884105726i128);
        assert_eq(250u8.add_wrapped(10u8), 4u8);
        assert_eq(65000u16.add_wrapped(1000u16), 464u16);
        assert_eq(4_294_967_290u32.add_wrapped(20u32), 14u32);
        assert_eq(18_446_744_073_709_551_600u64.add_wrapped(30u64), 14u64);
        assert_eq(340282366920938463463374607431768211440u128.add_wrapped(30u128), 14u128);

        // AND (&&) - bools only
        assert_eq(true && false, false);
        assert_eq(true && true, true);

        // OR (||) - bools only
        assert_eq(false || true, true);
        assert_eq(false || false, false);

        // NAND (.nand()) - bools only
        assert_eq(true.nand(true), false);
        assert_eq(false.nand(false), true);

        // NOR (.nor()) - bools only
        assert_eq(true.nor(false), false);
        assert_eq(false.nor(false), true);

        // BitwiseAnd (&) - bools and integers
        assert_eq(true & false, false);
        assert_eq(12i8 & 10i8, 8i8);
        assert_eq(1234i16 & 5678i16, 1026i16);
        assert_eq(123456i32 & 654321i32, 123456i32);
        assert_eq(9876543210i64 & 1234567890i64, 1217397442i64);
        assert_eq(314159265358979i128 & 271828182845904i128, 23338420404352i128);
        assert_eq(200u8 & 55u8, 0u8);
        assert_eq(50000u16 & 15535u16, 0u16);
        assert_eq(1000000000u32 & 3000000000u32, 848316928u32);
        assert_eq(9876543210987654321u64 & 1234567890123456789u64, 72058351590113297u64);
        assert_eq(170141183460469231731687303715884105727u128 & 1u128, 1u128);

        // BitwiseOr (|) - bools and integers
        assert_eq(true | false, true);
        assert_eq(12i8 | 10i8, 14i8);
        assert_eq(1234i16 | 5678i16, 5886i16);
        assert_eq(123456i32 | 654321i32, 654321i32);
        assert_eq(9876543210i64 | 1234567890i64, 9893713658i64);
        assert_eq(314159265358979i128 | 271828182845904i128, 562649027800531i128);
        assert_eq(200u8 | 55u8, 255u8);
        assert_eq(50000u16 | 15535u16, 65535u16);
        assert_eq(1000000000u32 | 3000000000u32, 3151683072u32);
        assert_eq(9876543210987654321u64 | 1234567890123456789u64, 11039052749520997813u64);
        assert_eq(170141183460469231731687303715884105727u128 | 1u128, 170141183460469231731687303715884105727u128);

        // XOR (^) - bools and integers
        assert_eq(true ^ false, true);
        assert_eq(12i8 ^ 10i8, 6i8);
        assert_eq(1234i16 ^ 5678i16, 4860i16);
        assert_eq(123456i32 ^ 654321i32, 530865i32);
        assert_eq(9876543210i64 ^ 1234567890i64, 8676316216i64);
        assert_eq(314159265358979i128 ^ 271828182845904i128, 539310607396179i128);
        assert_eq(200u8 ^ 55u8, 255u8);
        assert_eq(50000u16 ^ 15535u16, 65535u16);
        assert_eq(1000000000u32 ^ 3000000000u32, 2303366144u32);
        assert_eq(9876543210987654321u64 ^ 1234567890123456789u64, 10966994397930884516u64);
        assert_eq(170141183460469231731687303715884105727u128 ^ 1u128, 170141183460469231731687303715884105726u128);

        // Div
        assert_eq(4i8 / 2i8, 2i8);
        assert_eq(300i16 / 3i16, 100i16);
        assert_eq(10000i32 / 250i32, 40i32);
        assert_eq(9876543210i64 / 12345i64, 800044i64);
        assert_eq(314159265358979i128 / 271828i128, 1155728127i128);
        assert_eq(200u8 / 10u8, 20u8);
        assert_eq(65000u16 / 250u16, 260u16);
        assert_eq(3000000000u32 / 3u32, 1000000000u32);
        assert_eq(9876543210987654321u64 / 123456789u64, 80000000737u64);
        assert_eq(170141183460469231731687303715884105720u128 / 10u128, 17014118346046923173168730371588410572u128);
        assert_eq(8field / 2field, 4field);

        // Div Wrapped
        assert_eq((-128i8).div_wrapped(-1i8), -128i8);
        assert_eq((-32768i16).div_wrapped(-1i16), -32768i16);
        assert_eq((-2147483648i32).div_wrapped(-1i32), -2147483648i32);
        assert_eq((-9223372036854775808i64).div_wrapped(-1i64), -9223372036854775808i64);
        assert_eq((-170141183460469231731687303715884105728i128).div_wrapped(-1i128), -170141183460469231731687303715884105728i128);
        assert_eq(255u8.div_wrapped(5u8), 51u8);
        assert_eq(65535u16.div_wrapped(255u16), 257u16);
        assert_eq(4294967295u32.div_wrapped(3u32), 1431655765u32);
        assert_eq(18446744073709551615u64.div_wrapped(7u64), 2635249153387078802u64);
        assert_eq(340282366920938463463374607431768211455u128.div_wrapped(15u128), 22685491128062564230891640495451214097u128);

        // Eq
        assert_eq(true == true, true);
        assert_eq(false == true, false);
        assert_eq(42i8 == 42i8, true);
        assert_eq(-300i16 == -300i16, true);
        assert_eq(100000i32 == 100000i32, true);
        assert_eq(9876543210i64 == 9876543210i64, true);
        assert_eq(123456789012345678901234i128 == 123456789012345678901234i128, true);
        assert_eq(42u8 == 42u8, true);
        assert_eq(60000u16 == 60000u16, true);
        assert_eq(3000000000u32 == 3000000000u32, true);
        assert_eq(12345678901234567890u64 == 12345678901234567890u64, true);
        assert_eq(340282366920938463463374607431768211455u128 == 340282366920938463463374607431768211455u128, true);
        assert_eq([1u8, 2u8, 3u8] == [1u8, 2u8, 3u8], true);
        assert_eq([1i16, 2i16] == [1i16, 2i16], true);
        assert_eq(5field == 5field, true);
        assert_eq(2group == 2group, true);
        assert_eq(10scalar == 10scalar, true);
        assert_eq(aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4 
                  == aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4, true);
        assert_eq(Point { x: 1u32, y: 2u32 } == Point { x: 1u32, y: 2u32 }, true);
        assert_eq(Point { x: 1u32, y: 2u32 } == Point { y: 2u32, x: 1u32 }, true);

        // Neq
        assert_eq(true != false, true);
        assert_eq(false != true, true);
        assert_eq(42i8 != -42i8, true);
        assert_eq(-300i16 != 200i16, true);
        assert_eq(100000i32 != 100001i32, true);
        assert_eq(9876543210i64 != 9876543211i64, true);
        assert_eq(123456789012345678901234i128 != 123456789012345678901235i128, true);
        assert_eq(42u8 != 43u8, true);
        assert_eq(60000u16 != 60001u16, true);
        assert_eq(3000000000u32 != 3000000001u32, true);
        assert_eq(12345678901234567890u64 != 12345678901234567891u64, true);
        assert_eq(340282366920938463463374607431768211455u128 != 340282366920938463463374607431768211454u128, true);
        assert_eq([1u8, 2u8, 3u8] != [1u8, 2u8, 4u8], true);
        assert_eq([1i16, 2i16] != [2i16, 1i16], true);
        assert_eq(5field != 6field, true);
        assert_eq(2group != 0group, true);
        assert_eq(10scalar != 11scalar, true);
        assert_eq(   aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4 
                  != aleo1vtsjtursqdpsnyfp2ung0cm37x9p0v8g3erusaz4l2ch32at6y8qj8kgwu, true);
        assert_eq(Point { x: 1u32, y: 2u32 } != Point { x: 2u32, y: 1u32 }, true);
        assert_eq(Point { x: 1u32, y: 2u32 } != Point2 { x: 1u32, y: 2u32 }, true);

        // Less Than `<`
        assert_eq(1i8 < 2i8, true);
        assert_eq(300i16 < 400i16, true);
        assert_eq(10000i32 < 10001i32, true);
        assert_eq(9876543210i64 < 9876543211i64, true);
        assert_eq(314159265358979i128 < 314159265358980i128, true);
        assert_eq(42u8 < 43u8, true);
        assert_eq(60000u16 < 60001u16, true);
        assert_eq(3000000000u32 < 3000000001u32, true);
        assert_eq(12345678901234567890u64 < 12345678901234567891u64, true);
        assert_eq(340282366920938463463374607431768211454u128 < 340282366920938463463374607431768211455u128, true);
        assert_eq(5scalar < 6scalar, true);
        assert_eq(7field < 8field, true);

        // Less Or Equal `<=`
        assert_eq(2i8 <= 2i8, true);
        assert_eq(400i16 <= 400i16, true);
        assert_eq(10001i32 <= 10002i32, true);
        assert_eq(9876543211i64 <= 9876543211i64, true);
        assert_eq(314159265358980i128 <= 314159265358981i128, true);
        assert_eq(43u8 <= 43u8, true);
        assert_eq(60001u16 <= 60002u16, true);
        assert_eq(3000000001u32 <= 3000000002u32, true);
        assert_eq(12345678901234567891u64 <= 12345678901234567892u64, true);
        assert_eq(340282366920938463463374607431768211455u128 <= 340282366920938463463374607431768211455u128, true);
        assert_eq(6scalar <= 6scalar, true);
        assert_eq(8field <= 8field, true);

        // Greater Than `>`
        assert_eq(3i8 > 2i8, true);
        assert_eq(500i16 > 400i16, true);
        assert_eq(10002i32 > 10001i32, true);
        assert_eq(9876543212i64 > 9876543211i64, true);
        assert_eq(314159265358981i128 > 314159265358980i128, true);
        assert_eq(44u8 > 43u8, true);
        assert_eq(60002u16 > 60001u16, true);
        assert_eq(3000000002u32 > 3000000001u32, true);
        assert_eq(12345678901234567892u64 > 12345678901234567891u64, true);
        assert_eq(340282366920938463463374607431768211455u128 > 0u128, true);
        assert_eq(7scalar > 6scalar, true);
        assert_eq(9field > 8field, true);

        // Greater Or Equal `>=`
        assert_eq(3i8 >= 3i8, true);
        assert_eq(500i16 >= 400i16, true);
        assert_eq(10002i32 >= 10002i32, true);
        assert_eq(9876543212i64 >= 9876543211i64, true);
        assert_eq(314159265358981i128 >= 314159265358981i128, true);
        assert_eq(44u8 >= 43u8, true);
        assert_eq(60002u16 >= 60002u16, true);
        assert_eq(3000000002u32 >= 3000000001u32, true);
        assert_eq(12345678901234567892u64 >= 12345678901234567892u64, true);
        assert_eq(340282366920938463463374607431768211455u128 >= 0u128, true);
        assert_eq(7scalar >= 6scalar, true);
        assert_eq(9field >= 8field, true);

        // Mod
        assert_eq(10u8.mod(3u8), 1u8);
        assert_eq(60000u16.mod(256u16), 96u16);
        assert_eq(3000000000u32.mod(100000u32), 0u32);
        assert_eq(12345678901234567890u64.mod(1000000000u64), 234567890u64);
        assert_eq(340282366920938463463374607431768211455u128.mod(1000000000000000000000000000000000u128),366920938463463374607431768211455u128);

        // Mul
        assert_eq(3i8 * 4i8, 12i8);
        assert_eq(-300i16 * 2i16, -600i16);
        assert_eq(10000i32 * 3i32, 30000i32);
        assert_eq(123456789i64 * 2i64, 246913578i64);
        assert_eq(314159265358979i128 * 2i128, 628318530717958i128);
        assert_eq(10u8 * 20u8, 200u8);
        assert_eq(30000u16 * 2u16, 60000u16);
        assert_eq(1000000000u32 * 3u32, 3000000000u32);
        assert_eq(5000000000000000000u64 * 3u64, 15000000000000000000u64);
        assert_eq(340282366920938463463374607431768211455u128 * 1u128, 340282366920938463463374607431768211455u128);
        assert_eq(5field * 6field, 30field);
        assert_eq(123456789field * 2field, 246913578field);
        assert_eq(340282366920938463463374607431768211455field * 2field, 680564733841876926926749214863536422910field);
        assert_eq(2group * 5scalar, 2380684762819103024216024650390411236571559263701543273936065891954583563743group);
        assert_eq(5scalar * 2group, 2380684762819103024216024650390411236571559263701543273936065891954583563743group);

        // Mul Wrapped
        assert_eq(120i8.mul_wrapped(3i8), 104i8);
        assert_eq(32000i16.mul_wrapped(3i16), 30464i16);
        assert_eq(2147483640i32.mul_wrapped(2i32), -16i32);
        assert_eq(9223372036854775800i64.mul_wrapped(2i64), -16i64);
        assert_eq(170141183460469231731687303715884105720i128.mul_wrapped(2i128), -16i128);
        assert_eq(250u8.mul_wrapped(3u8), 238u8);
        assert_eq(65000u16.mul_wrapped(3u16), 63928u16);
        assert_eq(4294967290u32.mul_wrapped(3u32), 4294967278u32);
        assert_eq(18446744073709551600u64.mul_wrapped(3u64), 18446744073709551568u64);
        assert_eq(340282366920938463463374607431768211440u128.mul_wrapped(3u128), 340282366920938463463374607431768211408u128);

        // Pow
        assert_eq(2i8.pow(3u32), 8i8);
        assert_eq(3i16.pow(4u8), 81i16);
        assert_eq(5i32.pow(3u16), 125i32);
        assert_eq(7i64.pow(2u32), 49i64);
        assert_eq(2i128.pow(10u32), 1024i128);
        assert_eq(3u8.pow(5u32), 243u8);
        assert_eq(4u16.pow(3u32), 64u16);
        assert_eq(10u32.pow(4u32), 10000u32);
        assert_eq(2u64.pow(8u32), 256u64);
        assert_eq(5u128.pow(7u32), 78125u128);
        assert_eq(2field.pow(10field), 1024field);
        assert_eq(3field.pow(5field), 243field);
        assert_eq(5field.pow(0field), 1field);
        assert_eq(0field.pow(0field), 1field);

        // Pow Wrapped
        assert_eq(12i8.pow_wrapped(3u32), -64i8);
        assert_eq(1000i16.pow_wrapped(2u32), 16960i16);
        assert_eq(5000i16.pow_wrapped(3u32), -24064i16);
        assert_eq(70000i32.pow_wrapped(2u32), 605032704i32);
        assert_eq(300000000i32.pow_wrapped(3u32), -83886080i32);
        assert_eq(200u8.pow_wrapped(4u32), 0u8);
        assert_eq(250u8.pow_wrapped(3u32), 40u8);
        assert_eq(60000u16.pow_wrapped(3u32), 32768u16);
        assert_eq(3000000000u32.pow_wrapped(2u32), 3800301568u32);
        assert_eq(4000000000u32.pow_wrapped(3u32), 0u32);
        assert_eq(17000000000000000000u64.pow_wrapped(2u32), 7520607925660811264u64);
        assert_eq(340282366920938463463374607431768211440u128.pow_wrapped(2u32), 256u128);

        // Rem
        assert_eq(10i8.rem(3i8), 1i8);
        assert_eq(1000i16.rem(300i16), 100i16);
        assert_eq(123456i32.rem(1000i32), 456i32);
        assert_eq(9876543210i64.rem(12345i64), 30i64);
        assert_eq(314159265358979i128.rem(271828i128), 52823i128);
        assert_eq(200u8.rem(55u8), 35u8);
        assert_eq(50000u16.rem(15535u16), 3395u16);
        assert_eq(1000000000u32.rem(3000000000u32), 1000000000u32);
        assert_eq(9876543210987654321u64.rem(1234567890123456789u64), 90000000009u64);
        assert_eq(170141183460469231731687303715884105727u128.rem(1234567890123456789u128), 1063742247256300714u128);

        // Rem Wrapped
        assert_eq(10i8.rem_wrapped(3i8), 1i8);
        assert_eq(1000i16.rem_wrapped(300i16), 100i16);
        assert_eq(123456i32.rem_wrapped(1000i32), 456i32);
        assert_eq(9876543210i64.rem_wrapped(12345i64), 30i64);
        assert_eq(314159265358979i128.rem_wrapped(271828i128), 52823i128);
        assert_eq(200u8.rem_wrapped(55u8), 35u8);
        assert_eq(50000u16.rem_wrapped(15535u16), 3395u16);
        assert_eq(1000000000u32.rem_wrapped(3000000000u32), 1000000000u32);
        assert_eq(9876543210987654321u64.rem_wrapped(1234567890123456789u64), 90000000009u64);
        assert_eq(170141183460469231731687303715884105727u128.rem_wrapped(1234567890123456789u128), 1063742247256300714u128);

        // Shl
        assert_eq(1i8.shl(2u8), 4i8);
        assert_eq(16i16.shl(3u16), 128i16);
        assert_eq(1024i32.shl(5u32), 32768i32);
        assert_eq(1i64.shl(10u8), 1024i64);
        assert_eq(2i128.shl(8u16), 512i128);
        assert_eq(3u8.shl(3u8), 24u8);
        assert_eq(256u16.shl(4u16), 4096u16);
        assert_eq(1024u32.shl(10u32), 1048576u32);
        assert_eq(1u64.shl(63u8), 9223372036854775808u64);
        assert_eq(2u128.shl(100u16), 2535301200456458802993406410752u128);
        
        // Shl Wrapped
        assert_eq(1i8.shl_wrapped(10u8), 4i8);
        assert_eq(16i16.shl_wrapped(20u16), 256i16);
        assert_eq(1024i32.shl_wrapped(40u32), 262144i32);
        assert_eq(1i64.shl_wrapped(70u8), 64i64);
        assert_eq(2i128.shl_wrapped(200u16), 9444732965739290427392i128);
        assert_eq(3u8.shl_wrapped(11u8), 24u8);
        assert_eq(256u16.shl_wrapped(20u16), 4096u16);
        assert_eq(1024u32.shl_wrapped(40u32), 262144u32);
        assert_eq(1u64.shl_wrapped(130u8), 4u64);
        assert_eq(2u128.shl_wrapped(260u16), 32u128);

        // Shr
        assert_eq(8i8.shr(2u8), 2i8);
        assert_eq(256i16.shr(3u16), 32i16);
        assert_eq(32768i32.shr(5u32), 1024i32);
        assert_eq(1024i64.shr(10u8), 1i64);
        assert_eq(512i128.shr(8u16), 2i128);
        assert_eq(24u8.shr(3u8), 3u8);
        assert_eq(4096u16.shr(4u16), 256u16);
        assert_eq(1048576u32.shr(10u32), 1024u32);
        assert_eq(9223372036854775808u64.shr(63u8), 1u64);
        assert_eq(1267650600228229401496703205376u128.shr(100u16), 1u128);

        // Shr Wrapped
        assert_eq(8i8.shr_wrapped(10u8), 2i8);
        assert_eq(256i16.shr_wrapped(20u16), 16i16);
        assert_eq(32768i32.shr_wrapped(40u32), 128i32);
        assert_eq(1024i64.shr_wrapped(70u8), 16i64);
        assert_eq(512i128.shr_wrapped(200u16), 0i128);
        assert_eq(24u8.shr_wrapped(11u8), 3u8);
        assert_eq(4096u16.shr_wrapped(20u16), 256u16);
        assert_eq(1048576u32.shr_wrapped(40u32), 4096u32);
        assert_eq(9223372036854775808u64.shr_wrapped(130u8), 2305843009213693952u64);
        assert_eq(1267650600228229401496703205376u128.shr_wrapped(260u16), 79228162514264337593543950336u128);

        // Sub
        assert_eq(10i8 - 3i8, 7i8);
        assert_eq(1000i16 - 300i16, 700i16);
        assert_eq(123456i32 - 65432i32, 58024i32);
        assert_eq(9876543210i64 - 1234567890i64, 8641975320i64);
        assert_eq(314159265358979i128 - 271828182845904i128, 42331082513075i128);
        assert_eq(200u8 - 55u8, 145u8);
        assert_eq(50000u16 - 15535u16, 34465u16);
        assert_eq(1000000000u32 - 300000000u32, 700000000u32);
        assert_eq(9876543210987654321u64 - 1234567890123456789u64, 8641975320864197532u64);
        assert_eq(170141183460469231731687303715884105727u128 - 1234567890123456789u128, 170141183460469231730452735825760648938u128);
        assert_eq(10field - 5field, 5field);
        assert_eq(2group - 2group, 0group);

        // Sub Wrapped
        assert_eq(10i8.sub_wrapped(20i8), -10i8);
        assert_eq(1000i16.sub_wrapped(2000i16), -1000i16);
        assert_eq(2000i32.sub_wrapped(3000i32), -1000i32);
        assert_eq(9876543210i64.sub_wrapped(12345678900i64), -2469135690i64);
        assert_eq(314159265358979i128.sub_wrapped(500000000000000i128), -185840734641021i128);
        assert_eq(200u8.sub_wrapped(250u8), 206u8);
        assert_eq(50000u16.sub_wrapped(60000u16), 55536u16);
        assert_eq(1000000000u32.sub_wrapped(2000000000u32), 3294967296u32);
        assert_eq(9876543210987654321u64.sub_wrapped(12345678901234567890u64), 15977608383462638047u64);
        assert_eq(170141183460469231731687303715884105727u128.sub_wrapped(340282366920938463463374607431768211455u128),
        170141183460469231731687303715884105728u128);
    }
}
