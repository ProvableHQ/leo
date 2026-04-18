/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "update"
input = ["true", "{ x: 1u32, y: 2u32, z: 3u32 }", "{ x: 10u32, y: 20u32, z: 30u32 }"]
[case]
program = "test.aleo"
function = "update"
input = ["false", "{ x: 1u32, y: 2u32, z: 3u32 }", "{ x: 10u32, y: 20u32, z: 30u32 }"]
*/

struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    mapping points: u32 => Point;

    fn update(s: bool, a: Point, b: Point) -> Final {
        return final {
            let chosen: Point = s ? a : b;
            points.set(0u32, chosen);
        };
    }

    @noupgrade
    constructor() {}
}
