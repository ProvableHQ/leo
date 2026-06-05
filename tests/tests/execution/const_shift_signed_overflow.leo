/*
[case]
program = "const_shift_signed_overflow.aleo"
function = "const_overflow_i8"
input = []
*/

program const_shift_signed_overflow.aleo {
    fn const_overflow_i8() -> i8 {
        return 64i8 << 1u8;
    }

    @noupgrade
    constructor() {}
}
