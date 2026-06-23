struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn main(base: Point) -> u32 {
        // A struct update base must be the last entry inside the braces.
        let p: Point = Point { ..base, x: 1u32 };
        return p.x;
    }

    @noupgrade
    constructor() {}
}
