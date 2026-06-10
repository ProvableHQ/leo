struct S {
    x: u8,
    y: u32,
}

program test.aleo {
    // Regression: an array written element-wise then used directly in an assert must be rebuilt from its writes.
    fn write_then_assert(public p0: u8, public p1: u8) -> public u8 {
        let src: [u8; 4] = [p0, p1, p0, p1];
        let arr: [u8; 4] = [0u8; 4];
        for i: u32 in 0u32..4u32 {
            arr[i] = src[i];
        }
        assert_eq(arr, src);
        return p0;
    }

    // Same case for a struct written field-by-field.
    fn write_then_assert_struct(public a: u8, public b: u32) -> public u8 {
        let s: S = S { x: 0u8, y: 0u32 };
        s.x = a;
        s.y = b;
        assert_eq(s, S { x: a, y: b });
        return a;
    }

    @noupgrade
    constructor() {}
}
