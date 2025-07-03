program test.aleo {
    struct S {
        x: u8,
        y: u16,
    }

    transition main() {
        // Some tuples
        let t0 = (1u32, 2u16);
        let t0_0 = t0.0;

        let t1: (u32, u16) = (1, 2);
        let t1_1 = t1.1;

        let t2: (u32, u16) = (1u32, 2);

        // Struct field type inference works
        let s0 = S {
            x: 0, y: 0
        };

        let a0 = [1u16, 2u16];
        let a0_0 = a0[0u8];
        let a1: [u64; 3] = [1, 2, 3];
        let a2: [u64; 3] = [1u64, 2, 3];
        let a3: [u64; 3] = [1, 2u64, 3];

        let x: u32 = 1 + (true ? 0 : 1);
        let y: u32 = true ? 0 : 1;

        for i: u8 in 1..20 { }
        for i: u16 in 1..20 { }
        for i: u32 in 1..20 { }
        for i: u64 in 1..20 { }
        for i: u128 in 1..20 { }
        for i: i8 in 1..20 { }
        for i: i16 in 1..20 { }
        for i: i32 in 1..20 { }
        for i: i64 in 1..20 { }
        for i: i128 in 1..20 { }

        let z0 = bar(3, 4);
        let g = group::GEN;
    }

    inline bar(x: u32, y: u64) -> u16 {
        return 0 + 1;
    }
}
