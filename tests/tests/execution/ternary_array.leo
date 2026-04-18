/*
seed = 123456789

[case]
program = "test.aleo"
function = "select"
input = ["true", "[1u32, 2u32, 3u32, 4u32]", "[10u32, 20u32, 30u32, 40u32]"]
[case]
program = "test.aleo"
function = "select"
input = ["false", "[1u32, 2u32, 3u32, 4u32]", "[10u32, 20u32, 30u32, 40u32]"]
[case]
program = "test.aleo"
function = "select_element"
input = ["true", "[1u32, 2u32, 3u32, 4u32]", "[10u32, 20u32, 30u32, 40u32]"]
[case]
program = "test.aleo"
function = "select_element"
input = ["false", "[1u32, 2u32, 3u32, 4u32]", "[10u32, 20u32, 30u32, 40u32]"]
*/

program test.aleo {
    fn select(s: bool, a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        return s ? a : b;
    }

    fn select_element(s: bool, a: [u32; 4], b: [u32; 4]) -> u32 {
        return s ? a[0u32] : b[3u32];
    }

    @noupgrade
    constructor() {}
}
