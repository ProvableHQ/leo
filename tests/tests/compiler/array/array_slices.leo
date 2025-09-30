program test.aleo {
    transition simple_slices(a: [bool; 1]) -> bool {
        let b = a[..];
        let c = a[0..1];
        //let d = a[..1];
        //let e = a[0..];
        //let f = a[0..=0];
        //let g = a[..=0];

        assert_eq(b, c);
        //assert_eq(b, d);
        //assert_eq(b, e);
        //assert_eq(b, f);
        //assert_eq(b, g);

        return a[0];
    }
}
