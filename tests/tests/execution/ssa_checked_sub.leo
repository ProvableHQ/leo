/*
seed = 123456789

[case]
program = "ssa_checked_sub.aleo"
function = "main"
input = ["1u8", "1u8"]

[case]
program = "ssa_checked_sub.aleo"
function = "main"
input = ["0u8", "1u8"]
*/
program ssa_checked_sub.aleo {
    fn main(x: u8, y: u8) -> u8 {
        let diff: u8 = x - y;
        return diff * 0u8;
    }

    @noupgrade
    constructor() {}
}
