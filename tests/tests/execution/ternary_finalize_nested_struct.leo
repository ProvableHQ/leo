/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "update"
input = [
    "true",
    "{ lo: { x: 1u32, y: 2u32 }, hi: { x: 3u32, y: 4u32 }, tag: 5u32 }",
    "{ lo: { x: 10u32, y: 20u32 }, hi: { x: 30u32, y: 40u32 }, tag: 50u32 }",
]
[case]
program = "test.aleo"
function = "update"
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
    mapping segs: u32 => Segment;

    fn update(s: bool, a: Segment, b: Segment) -> Final {
        return final {
            let chosen: Segment = s ? a : b;
            segs.set(0u32, chosen);
        };
    }

    @noupgrade
    constructor() {}
}
