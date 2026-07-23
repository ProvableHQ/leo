struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn main(base: Point) -> u32 {
        // `..` must be followed by a base expression; `Point { .. }` is not valid.
        let p: Point = Point { .. };
        return p.x;
    }

    @noupgrade
    constructor() {}
}
