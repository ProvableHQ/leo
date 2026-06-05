/*
[case]
program = "const_shift_signed_valid.aleo"
function = "const_neg_shift"
input = []

[case]
program = "const_shift_signed_valid.aleo"
function = "const_min_shift"
input = []

[case]
program = "const_shift_signed_valid.aleo"
function = "runtime_min_shift"
input = ["-1i8"]
*/

program const_shift_signed_valid.aleo {
    fn const_neg_shift() -> i8 {
        return -2i8 << 1u8;
    }

    fn const_min_shift() -> i8 {
        return -1i8 << 7u8;
    }

    fn runtime_min_shift(x: i8) -> i8 {
        return x << 7u8;
    }

    @noupgrade
    constructor() {}
}
