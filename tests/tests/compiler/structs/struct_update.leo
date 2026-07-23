struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    fn main(x: u32) -> u32 {
        let base: Point = Point { x: 1u32, y: 2u32, z: 3u32 };
        // Override `x`, copy `y` and `z` from `base`.
        let updated: Point = Point { x: x, ..base };
        // Copy every field from `base`.
        let copy: Point = Point { ..base };
        return updated.x + updated.y + updated.z + copy.x + copy.y + copy.z;
    }

    @noupgrade
    constructor() {}
}
