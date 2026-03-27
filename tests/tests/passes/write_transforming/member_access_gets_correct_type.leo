struct S {
    x: u8,
    y: u32,
}

struct T {
    x: u8,
    y: [S; 3],
    z: S,
}

program test.aleo {
    fn nested2(s: S, t: T) -> (S, T) {
        t.y[1u8] = s;
        t.y[1u8].x = 1u8;
        return (s, t);
    }
}