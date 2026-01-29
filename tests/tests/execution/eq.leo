/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main"
input = ["0u32", "1u32"]
[case]
program = "test.aleo"
function = "main"
input = ["1u32", "1u32"]
[case]
program = "test.aleo"
function = "main"
input = ["1u32", "2u32"]
[case]
program = "test.aleo"
function = "main"
input = ["2u32", "2u32"]
*/

program test.aleo {
    fn main(x: u32, y: u32) -> bool {
        assert_eq(x, y);
        return x == y;
    }

    @noupgrade
    constructor() {}
}
