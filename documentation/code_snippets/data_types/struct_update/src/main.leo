// ANCHOR: file
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program struct_update.aleo {
    // Override `x`, and copy `y` and `z` from `base`.
    fn override_x(base: Point) -> Point {
        return Point { x: 10u32, ..base };
    }

    // Copy every field from `base`.
    fn copy_all(base: Point) -> Point {
        return Point { ..base };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
