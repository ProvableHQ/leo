export struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn origin() -> Point {
        return Point { x: 0u32, y: 0u32 };
    }
}

program test.aleo {
    fn main(a: u32) -> u32 {
        let p: Point = Point { x: a, y: a };
        return p.origin().x;
    }

    @noupgrade
    constructor() {}
}
