program test.aleo {
    fn defs_and_unary_ops() {
        let a0: u8 = 0;
        let a1: u16 = 0;
        let a2: u32 = 0;
        let a3: u64 = 0;
        let a4: u128 = 0;
        let a5: i8 = 0;
        let a6: i16 = 0;
        let a7: i32 = 0;
        let a8: i64 = 0;
        let a9: i128 = 0;
        let a10: scalar = 0;
        let a11: group = 0;
        let a12: field = 0;
        let a13: bool = true;

        // Bitwise AND
        let _b0 = a0 & 1;
        let _b1 = a1 & 1u16;
        let _b2 = 1 & a2;
        let _b3 = 1u64 & a3;
        let _b4: u128 = a4 & 1;
        let _b5: i8 = a5 & 1i8;
        let _b6: i16 = 1 & a6;
        let _b7: i32 = 1i32 & a7;
        let _b8: i64 = 1 & 1;
        let _b9: i128 = 1i128 & 1i128;
        let _b10: bool = true & a13;

        // Bitwise OR
        let _c0 = a0 | 1;
        let _c1 = a1 | 1u16;
        let _c2 = 1 | a2;
        let _c3 = 1u64 | a3;
        let _c4: u128 = a4 | 1;
        let _c5: i8 = a5 | 1i8;
        let _c6: i16 = 1 | a6;
        let _c7: i32 = 1i32 | a7;
        let _c8: i64 = 1 | 1;
        let _c9: i128 = 1i128 | 1i128;
        let _c10: bool = true | a13;

        // Bitwise XOR
        let _d0 = a0 ^ 1;
        let _d1 = a1 ^ 1u16;
        let _d2 = 1 ^ a2;
        let _d3 = 1u64 ^ a3;
        let _d4: u128 = a4 ^ 1;
        let _d5: i8 = a5 ^ 1i8;
        let _d6: i16 = 1 ^ a6;
        let _d7: i32 = 1i32 ^ a7;
        let _d8: i64 = 1 ^ 1;
        let _d9: i128 = 1i128 ^ 1i128;
        let _d10: bool = true ^ a13;

        // Addition
        let _e0 = a0 + 1;
        let _e1 = a1 + 1u16;
        let _e2 = 1 + a2;
        let _e3 = 1u64 + a3;
        let _e4: u128 = a4 + 1;
        let _e5: i8 = a5 + 1i8;
        let _e6: i16 = 1 + a6;
        let _e7: i32 = 1i32 + a7;
        let _e8: i64 = 1 + 1;
        let _e9: i128 = 1i128 + 1i128;
        let _e10 = a10 + 1;
        let _e11 = 0 + a11;
        let _e12 = 1 + a12;

        // Subtraction
        let _f0 = a0 - 1;
        let _f1 = a1 - 1u16;
        let _f2 = 1 - a2;
        let _f3 = 1u64 - a3;
        let _f4: u128 = a4 - 1;
        let _f5: i8 = a5 - 1i8;
        let _f6: i16 = 1 - a6;
        let _f7: i32 = 1i32 - a7;
        let _f8: i64 = 1 - 1;
        let _f9: i128 = 1i128 - 1i128;
        let _f10 = a11 - 0;
        let _f11 = 1 - a12;

        // Multiplication
        let _g0 = a0 * 1;
        let _g1 = a1 * 1u16;
        let _g2 = 1 * a2;
        let _g3 = 1u64 * a3;
        let _g4: u128 = a4 * 1;
        let _g5: i8 = a5 * 1i8;
        let _g6: i16 = 1 * a6;
        let _g7: i32 = 1i32 * a7;
        let _g8: i64 = 1 * 1;
        let _g9: i128 = 1i128 * 1i128;
        let _g10 = a11 * 1; // infer 1 as a scalar
        let _g11 = 1 * a11; // infer 1 as a scalar
        let _g12 = a10 * 2; // infer 2 as a group
        let _g13 = 2 * a10; // infer 2 as a group
        let _g14 = a12 * 1;

        // Division
        let _h0 = a0 / 1;
        let _h1 = a1 / 1u16;
        let _h2 = 1 / a2;
        let _h3 = 1u64 / a3;
        let _h4: u128 = a4 / 1;
        let _h5: i8 = a5 / 1i8;
        let _h6: i16 = 1 / a6;
        let _h7: i32 = 1i32 / a7;
        let _h8: i64 = 1 / 1;
        let _h9: i128 = 1i128 / 1i128;
        let _h10 = a12 / 1;

        // Rem
        let _i0 = a0 % 1;
        let _i1 = a1 % 1u16;
        let _i2 = 1 % a2;
        let _i3 = 1u64 % a3;
        let _i4: u128 = a4 % 1;
        let _i5: i8 = a5 % 1i8;
        let _i6: i16 = 1 % a6;
        let _i7: i32 = 1i32 % a7;
        let _i8_: i64 = 1 % 1;
        let _i9: i128 = 1i128 % 1i128;

        // Rem Wrapped
        let _j0 = a0.rem_wrapped(1);
        let _j1 = a1.rem_wrapped(1u16);
        let _j2 = 1.rem_wrapped(a2);
        let _j3 = 1u64.rem_wrapped(a3);
        let _j4: u128 = a4.rem_wrapped(1);
        let _j5: i8 = a5.rem_wrapped(1i8);
        let _j6: i16 = 1.rem_wrapped(a6);
        let _j7: i32 = 1i32.rem_wrapped(a7);
        let _j8: i64 = 1.rem_wrapped(1);
        let _j9: i128 = 1i128.rem_wrapped(1i128);

        // mod operator (unsigned integers only)
        let _k0 = a0.mod(1);
        let _k1 = a1.mod(1u16);
        let _k2 = 1.mod(a2);
        let _k3 = 1u64.mod(a3);
        let _k4: u128 = a4.mod(1);
        let _k5: u8 = a0.mod(1u8);
        let _k6: u16 = 1.mod(a1);
        let _k7: u32 = 1u32.mod(a2);
        let _k8: u64 = 1.mod(1);
        let _k9: u128 = 1u128.mod(1u128);

        // Pow
        let _l0 = a0 ** 1u8;
        let _l1 = a1 ** 1u16;
        let _l2: u8 = 1 ** a2;
        let _l3 = 1u64 ** a2;
        let _l4: u128 = a4 ** 1u32;
        let _l5: i8 = a5 ** 1u8;
        let _l6: i16 = 1 ** a1;
        let _l7: i32 = 1i32 ** a2;
        let _l8: i64 = 1 ** 1u8;
        let _l9: i128 = 1i128 ** 1u32;
        let _l10: field = 1 ** 1; // infer the 1s as `field`
        let _l11 = 1field ** 1; // infer 1 as `field`
        let _l12 = 1 ** 1field; // infer 1 as `field`
        // Pow Wrapped
        let _m0 = a0.pow_wrapped(1u8);
        let _m1 = a1.pow_wrapped(1u16);
        let _m2: u8 = 1.pow_wrapped(a2);
        let _m3 = 1u64.pow_wrapped(a2);
        let _m4: u128 = a4.pow_wrapped(1u32);
        let _m5: i8 = a5.pow_wrapped(1u8);
        let _m6: i16 = 1.pow_wrapped(a1);
        let _m7: i32 = 1i32.pow_wrapped(a2);
        let _m8: i64 = 1.pow_wrapped(1u8);
        let _m9: i128 = 1i128.pow_wrapped(1u32);

        // Eq, Neq
        let _n0 = a0 == 1;
        let _n1 = a1 != 1;
        let _n2 = 1 == a2;
        let _n3 = 1 != a3;
        let _n4 = a4 == 1;
        let _n5 = a5 != 1;
        let _n6 = 1 == a6;
        let _n7 = 1 != a7;
        let _n8 = 1 == a8;
        let _n9 = a9 != 1;
        let _n10 = 1 == a10;
        let _n11 = 0 != a11;
        let _n12 = 1 == a12;

        // Lt, Gt, Lte, Gte
        let _o0 = a0 > 1;
        let _o1 = a1 < 1;
        let _o2 = 1 <= a2;
        let _o3 = 1 >= a3;
        let _o4 = a4 > 1;
        let _o5 = a5 < 1;
        let _o6 = 1 <= a6;
        let _o7 = 1 >= a7;
        let _o8 = 1 > a8;
        let _o9 = a9 < 1;
        let _o10 = 1 <= a10;
        let _o12 = 1 > a12;

        // add_wrapped (integers only)
        let _p0 = a0.add_wrapped(1);
        let _p1 = a1.add_wrapped(1u16);
        let _p2 = 1.add_wrapped(a2);
        let _p3 = 1u64.add_wrapped(a3);
        let _p4: u128 = a4.add_wrapped(1);
        let _p5: i8 = a5.add_wrapped(1i8);
        let _p6: i16 = 1.add_wrapped(a6);
        let _p7: i32 = 1i32.add_wrapped(a7);
        let _p8: i64 = 1.add_wrapped(1);
        let _p9: i128 = 1i128.add_wrapped(1i128);

        // sub_wrapped (integers only)
        let _q0 = a0.sub_wrapped(1);
        let _q1 = a1.sub_wrapped(1u16);
        let _q2 = 1.sub_wrapped(a2);
        let _q3 = 1u64.sub_wrapped(a3);
        let _q4: u128 = a4.sub_wrapped(1);
        let _q5: i8 = a5.sub_wrapped(1i8);
        let _q6: i16 = 1.sub_wrapped(a6);
        let _q7: i32 = 1i32.sub_wrapped(a7);
        let _q8: i64 = 1.sub_wrapped(1);
        let _q9: i128 = 1i128.sub_wrapped(1i128);

        // mul_wrapped (integers only)
        let _r0 = a0.mul_wrapped(1);
        let _r1 = a1.mul_wrapped(1u16);
        let _r2 = 1.mul_wrapped(a2);
        let _r3 = 1u64.mul_wrapped(a3);
        let _r4: u128 = a4.mul_wrapped(1);
        let _r5: i8 = a5.mul_wrapped(1i8);
        let _r6: i16 = 1.mul_wrapped(a6);
        let _r7: i32 = 1i32.mul_wrapped(a7);
        let _r8: i64 = 1.mul_wrapped(1);
        let _r9: i128 = 1i128.mul_wrapped(1i128);

        // div_wrapped (integers only)
        let _s0 = a0.div_wrapped(1);
        let _s1 = a1.div_wrapped(1u16);
        let _s2 = 1.div_wrapped(a2);
        let _s3 = 1u64.div_wrapped(a3);
        let _s4: u128 = a4.div_wrapped(1);
        let _s5: i8 = a5.div_wrapped(1i8);
        let _s6: i16 = 1.div_wrapped(a6);
        let _s7: i32 = 1i32.div_wrapped(a7);
        let _s8: i64 = 1.div_wrapped(1);
        let _s9: i128 = 1i128.div_wrapped(1i128);

        // Bitwise left shift `<<`
        let _t0 = a0 << 1u8;
        let _t1 = a1 << 1u16;
        let _t2: u8 = 1 << a2;
        let _t3 = 1u64 << a2;
        let _t4: u128 = a4 << 1u32;
        let _t5: i8 = a5 << 1u8;
        let _t6: i16 = 1 << a1;
        let _t7: i32 = 1i32 << a2;
        let _t8: i64 = 1 << 1u8;
        let _t9: i128 = 1i128 << 1u32;

        // shl_wrapped
        let _u0 = a0.shl_wrapped(1u8);
        let _u1 = a1.shl_wrapped(1u16);
        let _u2: u8 = 1.shl_wrapped(a2);
        let _u3 = 1u64.shl_wrapped(a2);
        let _u4: u128 = a4.shl_wrapped(1u32);
        let _u5: i8 = a5.shl_wrapped(1u8);
        let _u6: i16 = 1.shl_wrapped(a1);
        let _u7: i32 = 1i32.shl_wrapped(a2);
        let _u8_: i64 = 1.shl_wrapped(1u8);
        let _u9: i128 = 1i128.shl_wrapped(1u32);

        // Bitwise right shift `>>`
        let _v0 = a0 >> 1u8;
        let _v1 = a1 >> 1u16;
        let _v2: u8 = 1 >> a2;
        let _v3 = 1u64 >> a2;
        let _v4: u128 = a4 >> 1u32;
        let _v5: i8 = a5 >> 1u8;
        let _v6: i16 = 1 >> a1;
        let _v7: i32 = 1i32 >> a2;
        let _v8: i64 = 1 >> 1u8;
        let _v9: i128 = 1i128 >> 1u32;

        // shr_wrapped
        let _w0 = a0.shr_wrapped(1u8);
        let _w1 = a1.shr_wrapped(1u16);
        let _w2: u8 = 1.shr_wrapped(a2);
        let _w3 = 1u64.shr_wrapped(a2);
        let _w4: u128 = a4.shr_wrapped(1u32);
        let _w5: i8 = a5.shr_wrapped(1u8);
        let _w6: i16 = 1.shr_wrapped(a1);
        let _w7: i32 = 1i32.shr_wrapped(a2);
        let _w8: i64 = 1.shr_wrapped(1u8);
        let _w9: i128 = 1i128.shr_wrapped(1u32);
    }

    fn unary_ops() {
        let _b0: i8 = -1.abs();
        let _b1: i16 = -1.abs_wrapped();
        let _b2: field = 0.double();
        let _b3: group = 0.double();
        let _b4 = 2.inv();
        let _b5: field = 2.inv();
        let _b6: i32 = -2;
        let _b7: i64 = !2;
        let _b8 = 2.square(); // 2 must be field
        let _b9: field = 2.square(); // 2 must be field
        let _b10 = 2.square_root(); // 2 must be field
        let _b11: field = 2.square_root(); // 2 must be field
        let _b12 = 2.to_x_coordinate(); // 2 must be field
        let _b13: field = 2.to_x_coordinate(); // 2 must be field
        let _b14 = 2.to_y_coordinate(); // 2 must be group
        let _b15: field = 2.to_y_coordinate(); // 2 must be group
    }

    @noupgrade
    constructor() {}
}
