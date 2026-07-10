struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn main(base: Point) -> u32 {
        // `w` is not a field of `Point`.
        let updated: Point = Point { w: 5u32, ..base };
        return updated.x;
    }

    @noupgrade
    constructor() {}
}
