/*
seed = 123456789

[case]
program = "subtraction_before_finalize_state_write.aleo"
function = "checked_sub"
input = ["0u8"]
[case]
program = "subtraction_before_finalize_state_write.aleo"
function = "checked_sub"
input = ["1u8"]
*/

program subtraction_before_finalize_state_write.aleo {
    fn checked_sub(amount: u8) -> u8 {
        return 0u8 - amount;
    }

    @noupgrade
    constructor() {}
}
