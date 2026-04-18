struct Point {
    x: u32,
    y: u32,
}

struct Segment {
    lo: Point,
    hi: Point,
    tag: u32,
}

program test.aleo {
    fn select(s: bool, a: Segment, b: Segment) -> Segment {
        return s ? a : b;
    }

    fn select_inner(s: bool, a: Segment, b: Segment) -> Point {
        return s ? a.lo : b.hi;
    }
}
