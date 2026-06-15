struct S {
    x: u8,
    y: u16,
}

program test.aleo {
    fn aggregates() {
        // Some tuples
        let t0 = (1u32, 2u16);
        let _t0_0 = t0.0;

        let t1: (u32, u16) = (1, 2);
        let _t1_1 = t1.1;

        let _t2: (u32, u16) = (1u32, 2);

        // Struct field type inference works
        let _s0 = S { x: 0, y: 0 };
        let a0 = [1u16, 2u16];
        let _a0_0 = a0[0u8];
        let _a1: [u64; 3] = [1, 2, 3];
        let _a2: [u64; 3] = [1u64, 2, 3];
        let _a3: [u64; 3] = [1, 2u64, 3];

        let _x: u32 = 1 + (true ? 0 : 1);
        let _y: u32 = true ? 0 : 1;
        for _i: u8 in 1..20 {}
        for _i: u16 in 1..20 {}
        for _i: u32 in 1..20 {}
        for _i: u64 in 1..20 {}
        for _i: u128 in 1..20 {}
        for _i: i8 in 1..20 {}
        for _i: i16 in 1..20 {}
        for _i: i32 in 1..20 {}
        for _i: i64 in 1..20 {}
        for _i: i128 in 1..20 {}
    }

    fn function_calls(s: signature, a: address) {
        let _z0 = bar(3, 4);
        let _g = group::GEN;
        let _verify = signature::verify(s, a, 0field);
    }

    @noupgrade
    constructor() {}
}

fn bar(_x: u32, _y: u64) -> u16 {
    return 0 + 1;
}
