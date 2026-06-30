struct Point {
    x: u32,
    y: u32,
}

struct Other {
    x: u32,
    y: u32,
}

program test.aleo {
    fn main(x: u32) -> u32 {
        let base: Other = Other { x: 1u32, y: 2u32 };
        // The base has a different composite type.
        let updated: Point = Point { x: x, ..base };
        return updated.x;
    }

    @noupgrade
    constructor() {}
}
