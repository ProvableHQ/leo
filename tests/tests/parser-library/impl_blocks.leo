
struct Point { x: u32, y: u32 }

impl Point {
    fn new(x: u32, y: u32) -> Point { return Point { x, y }; }
    fn sum(self) -> u32 { return self.x + self.y; }
}
