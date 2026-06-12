/*
seed = 123456789

[case]
program = "subtraction_underflow_halts.aleo"
function = "checked_sub"
input = ["0u8"]
[case]
program = "subtraction_underflow_halts.aleo"
function = "checked_sub"
input = ["1u8"]
*/

program subtraction_underflow_halts.aleo {
    fn checked_sub(amount: u8) -> u8 {
        return 0u8 - amount;
    }

    @noupgrade
    constructor() {}
}
