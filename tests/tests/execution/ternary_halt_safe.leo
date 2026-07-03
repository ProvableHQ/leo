/*
seed = 123456789

[case]
program = "test.aleo"
function = "divide"
input = ["false", "0u8"]
[case]
program = "test.aleo"
function = "divide"
input = ["true", "5u8"]
[case]
program = "test.aleo"
function = "divide"
input = ["true", "0u8"]
[case]
program = "test.aleo"
function = "add_overflow"
input = ["false", "255u8"]
[case]
program = "test.aleo"
function = "shift"
input = ["false", "200u8"]
[case]
program = "test.aleo"
function = "narrow_cast"
input = ["false", "100000u32"]
[case]
program = "test.aleo"
function = "nested"
input = ["false", "false", "0u8"]
[case]
program = "test.aleo"
function = "nested"
input = ["true", "true", "0u8"]
[case]
program = "test.aleo"
function = "signed_divide"
input = ["false", "-128i8", "-1i8"]
[case]
program = "test.aleo"
function = "signed_divide"
input = ["true", "-128i8", "-1i8"]
*/

program test.aleo {
    // The untaken arm `10u8 / b` would divide by zero when `b == 0`,
    // but predication keeps it halt-safe so `99u8` is returned.
    // When the arm is taken with `b == 0`, the genuine fault still halts.
    fn divide(c: bool, b: u8) -> u8 {
        return c ? 10u8 / b : 99u8;
    }

    // The untaken arm would overflow `a + 1u8` when `a == 255u8`.
    fn add_overflow(c: bool, a: u8) -> u8 {
        return c ? a + 1u8 : 7u8;
    }

    // The untaken arm would shift by more than the bit width.
    fn shift(c: bool, s: u8) -> u8 {
        return c ? 1u8 << s : 3u8;
    }

    // The untaken arm would overflow the narrowing cast `x as u8`.
    fn narrow_cast(c: bool, x: u32) -> u8 {
        return c ? x as u8 : 42u8;
    }

    // Nested ternaries: the deep `10u8 / b` is guarded by `c1 && c2`,
    // so the untaken case is safe, while taking both arms with `b == 0` halts.
    fn nested(c1: bool, c2: bool, b: u8) -> u8 {
        return c1 ? (c2 ? 10u8 / b : 1u8) : 2u8;
    }

    // Signed `MIN / -1` also halts; predicating the divisor to `1` covers it in the
    // untaken arm, while the taken arm still halts on the genuine overflow.
    fn signed_divide(c: bool, a: i8, b: i8) -> i8 {
        return c ? a / b : 0i8;
    }

    @noupgrade
    constructor() {}
}
