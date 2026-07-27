/*
seed = 123456789
min_height = 16

[case]
program = "tuple_assert_neq.aleo"
function = "main"
input = ["1u32", "2u32", "1u32", "3u32"]

[case]
program = "tuple_assert_neq.aleo"
function = "main"
input = ["1u32", "2u32", "9u32", "2u32"]

[case]
program = "tuple_assert_neq.aleo"
function = "main"
input = ["1u32", "2u32", "9u32", "8u32"]

[case]
program = "tuple_assert_neq.aleo"
function = "main"
input = ["1u32", "2u32", "1u32", "2u32"]
*/

program tuple_assert_neq.aleo {
    fn main(a0: u32, a1: u32, b0: u32, b1: u32) -> Final {
        return final {
            assert_neq((a0, a1), (b0, b1));
        };
    }

    @noupgrade
    constructor() {}
}
