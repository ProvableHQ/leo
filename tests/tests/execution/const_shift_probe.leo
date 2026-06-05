/*
[case]
program = "const_shift_probe.aleo"
function = "const_overflow_u8"
input = []
*/

program const_shift_probe.aleo {
    fn const_overflow_u8() -> u8 {
        return 128u8 << 1u8;
    }

    @noupgrade
    constructor() {}
}
