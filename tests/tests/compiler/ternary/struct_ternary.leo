struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    fn select(s: bool, a: Point, b: Point) -> Point {
        return s ? a : b;
    }
}
