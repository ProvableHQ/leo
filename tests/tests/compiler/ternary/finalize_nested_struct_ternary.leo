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
    mapping segs: u32 => Segment;

    fn update(s: bool, a: Segment, b: Segment) -> Final {
        return final {
            let chosen: Segment = s ? a : b;
            segs.set(0u32, chosen);
        };
    }
}
