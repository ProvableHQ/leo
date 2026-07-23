/*
seed = 123456789

[case]
program = "test.aleo"
function = "override_one"
input = ["{ x: 1u32, y: 2u32, z: 3u32 }", "10u32"]
[case]
program = "test.aleo"
function = "copy_all"
input = ["{ x: 4u32, y: 5u32, z: 6u32 }"]
[case]
program = "test.aleo"
function = "override_all"
input = ["{ x: 1u32, y: 1u32, z: 1u32 }", "7u32", "8u32", "9u32"]
[case]
program = "test.aleo"
function = "nested"
input = ["{ inner: { a: 1u32, b: 2u32 }, tag: 3u32 }", "42u32"]
*/

struct Inner {
    a: u32,
    b: u32,
}

struct Point {
    x: u32,
    y: u32,
    z: u32,
}

struct Outer {
    inner: Inner,
    tag: u32,
}

program test.aleo {
    // Override one field; copy the rest from a runtime base.
    fn override_one(base: Point, new_x: u32) -> Point {
        return Point { x: new_x, ..base };
    }

    // Copy every field from a runtime base.
    fn copy_all(base: Point) -> Point {
        return Point { ..base };
    }

    // Every field is overridden, so nothing is copied from `base`.
    fn override_all(base: Point, a: u32, b: u32, c: u32) -> Point {
        return Point { x: a, y: b, z: c, ..base };
    }

    // Nested update: the inner struct is built with its own update from `base.inner`,
    // and the outer update copies `tag` from `base`.
    fn nested(base: Outer, new_a: u32) -> Outer {
        let new_inner: Inner = Inner { a: new_a, ..base.inner };
        return Outer { inner: new_inner, ..base };
    }

    @noupgrade
    constructor() {}
}
