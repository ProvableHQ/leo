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
}
