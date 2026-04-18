/*
seed = 123456789

[case]
program = "test.aleo"
function = "select"
input = [
    "true",
    "{ lo: { x: 1u32, y: 2u32 }, hi: { x: 3u32, y: 4u32 }, tag: 5u32 }",
    "{ lo: { x: 10u32, y: 20u32 }, hi: { x: 30u32, y: 40u32 }, tag: 50u32 }",
]
[case]
program = "test.aleo"
function = "select"
input = [
    "false",
    "{ lo: { x: 1u32, y: 2u32 }, hi: { x: 3u32, y: 4u32 }, tag: 5u32 }",
    "{ lo: { x: 10u32, y: 20u32 }, hi: { x: 30u32, y: 40u32 }, tag: 50u32 }",
]
*/

struct Point {
    x: u32,
    y: u32,
}

struct Segment {
    lo: Point,
    hi: Point,
    tag: u32,
}

program test.aleo {
    fn select(s: bool, a: Segment, b: Segment) -> Segment {
        return s ? a : b;
    }

    @noupgrade
    constructor() {}
}
