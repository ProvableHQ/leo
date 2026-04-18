/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "update"
input = [
    "true",
    "[{ x: 1u32, y: 2u32 }, { x: 3u32, y: 4u32 }]",
    "[{ x: 10u32, y: 20u32 }, { x: 30u32, y: 40u32 }]",
]
[case]
program = "test.aleo"
function = "update"
input = [
    "false",
    "[{ x: 1u32, y: 2u32 }, { x: 3u32, y: 4u32 }]",
    "[{ x: 10u32, y: 20u32 }, { x: 30u32, y: 40u32 }]",
]
*/

struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    mapping arr: u32 => [Point; 2];

    fn update(s: bool, a: [Point; 2], b: [Point; 2]) -> Final {
        return final {
            let chosen: [Point; 2] = s ? a : b;
            arr.set(0u32, chosen);
        };
    }

    @noupgrade
    constructor() {}
}
