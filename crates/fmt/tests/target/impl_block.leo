export struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        return Point { x, y };
    }

    fn sum(self) -> u32 {
        return self.x + self.y;
    }
}

program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        let p: Point = Point::new(a, b);
        return p.sum();
    }

    @noupgrade
    constructor() {}
}
