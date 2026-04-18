struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    fn select(s: bool, a: [Point; 4], b: [Point; 4]) -> [Point; 4] {
        return s ? a : b;
    }

    fn select_element(s: bool, a: [Point; 4], b: [Point; 4]) -> Point {
        return s ? a[0u32] : b[1u32];
    }
}
