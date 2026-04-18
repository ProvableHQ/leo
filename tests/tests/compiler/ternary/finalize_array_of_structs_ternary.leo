struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    mapping arr: u32 => [Point; 3];

    fn update(s: bool, a: [Point; 3], b: [Point; 3]) -> Final {
        return final {
            let chosen: [Point; 3] = s ? a : b;
            arr.set(0u32, chosen);
        };
    }
}
