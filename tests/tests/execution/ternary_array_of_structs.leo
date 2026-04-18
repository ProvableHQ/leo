/*
seed = 123456789

[case]
program = "test.aleo"
function = "select"
input = [
    "true",
    "[{ x: 1u32, y: 2u32 }, { x: 3u32, y: 4u32 }, { x: 5u32, y: 6u32 }]",
    "[{ x: 10u32, y: 20u32 }, { x: 30u32, y: 40u32 }, { x: 50u32, y: 60u32 }]",
]
[case]
program = "test.aleo"
function = "select"
input = [
    "false",
    "[{ x: 1u32, y: 2u32 }, { x: 3u32, y: 4u32 }, { x: 5u32, y: 6u32 }]",
    "[{ x: 10u32, y: 20u32 }, { x: 30u32, y: 40u32 }, { x: 50u32, y: 60u32 }]",
]
*/

struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn select(s: bool, a: [Point; 3], b: [Point; 3]) -> [Point; 3] {
        return s ? a : b;
    }

    @noupgrade
    constructor() {}
}
