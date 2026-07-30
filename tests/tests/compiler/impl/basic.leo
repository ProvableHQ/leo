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

    fn scaled(self, factor: u32) -> Point {
        return Point::new(self.x * factor, self.y * factor);
    }
}

program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        let p: Point = Point::new(a, b);
        let q: Point = p.scaled(2u32);
        return q.sum();
    }

    @noupgrade
    constructor() {}
}
