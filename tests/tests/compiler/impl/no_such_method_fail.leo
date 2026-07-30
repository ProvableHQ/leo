export struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn sum(self) -> u32 {
        return self.x + self.y;
    }
}

program test.aleo {
    fn main(a: u32) -> u32 {
        let p: Point = Point { x: a, y: a };
        return p.nonexistent();
    }

    @noupgrade
    constructor() {}
}
