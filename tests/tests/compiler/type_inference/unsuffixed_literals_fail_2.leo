program test.aleo {
    transition foo() {
        let t0 = (1, 2);
        let t1: u32 = (1, 2);
        let t2_0 = (1, 2).0;
        let a0 = [1, 2];
        let a1: u32 = [1, 2];
        let a1_0 = a1[0];
        let a2_0 = [1, 2][0u32];
        let verify = signature::verify(0, 0, 0);
        let t = true ? 0 : 1;
        let c = 2 as u16;
        for i in 1..2 { }
        assert_eq(1, 1);

        // non-decimal error
        let f: field = 0x0;
        let s: scalar = 0b0;
        let g: group = 0o0;
    }
}
