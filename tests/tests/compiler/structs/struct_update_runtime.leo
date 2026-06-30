struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    fn main(base: Point, new_x: u32) -> Point {
        // `base` is a runtime value, so the missing fields are read from it via member accesses.
        return Point { x: new_x, ..base };
    }

    @noupgrade
    constructor() {}
}
