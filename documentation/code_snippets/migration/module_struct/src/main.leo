// ANCHOR: file
struct Point {
    x: i32,
    y: i32,
}

program test.aleo {
    fn foo(p: Point) -> Point {
        return Point { x: p.y, y: p.x };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
