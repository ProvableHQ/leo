struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn main(base: Point) -> u32 {
        // `x` is specified twice; with a base present this previously slipped through silently and
        // SSA kept only the last value. It must be rejected as a duplicate field.
        let p: Point = Point { x: 1u32, x: 2u32, ..base };
        return p.x;
    }

    @noupgrade
    constructor() {}
}
