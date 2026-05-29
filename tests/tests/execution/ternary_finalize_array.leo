/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "update"
input = ["true", "[1u32, 2u32, 3u32, 4u32]", "[10u32, 20u32, 30u32, 40u32]"]
[case]
program = "test.aleo"
function = "update"
input = ["false", "[1u32, 2u32, 3u32, 4u32]", "[10u32, 20u32, 30u32, 40u32]"]
*/

program test.aleo {
    mapping arr: u32 => [u32; 4];

    fn update(s: bool, a: [u32; 4], b: [u32; 4]) -> Final {
        return final {
            let chosen: [u32; 4] = s ? a : b;
            arr.set(0u32, chosen);
        };
    }

    @noupgrade
    constructor() {}
}
