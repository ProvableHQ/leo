export struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn add(self, other: Point) -> u32 {
        return self.x + other.x;
    }
}

program test.aleo {
    fn main(a: u32) -> u32 {
        return a;
    }

    @noupgrade
    constructor() {}
}
