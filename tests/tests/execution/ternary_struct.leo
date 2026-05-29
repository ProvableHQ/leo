/*
seed = 123456789

[case]
program = "test.aleo"
function = "select"
input = ["true", "{ x: 1u32, y: 2u32, z: 3u32 }", "{ x: 10u32, y: 20u32, z: 30u32 }"]
[case]
program = "test.aleo"
function = "select"
input = ["false", "{ x: 1u32, y: 2u32, z: 3u32 }", "{ x: 10u32, y: 20u32, z: 30u32 }"]
*/

struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    fn select(s: bool, a: Point, b: Point) -> Point {
        return s ? a : b;
    }

    @noupgrade
    constructor() {}
}
