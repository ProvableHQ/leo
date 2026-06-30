struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn main(base: Point, a: u32, b: u32) -> Point {
        // Every field is overridden, so nothing is copied from `base`, yet `base` is still a valid
        // base of the right type and the update compiles. Exercises the "base fully overridden" path.
        return Point { x: a, y: b, ..base };
    }

    @noupgrade
    constructor() {}
}
