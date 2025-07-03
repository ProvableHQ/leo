program test.aleo {
    transition main() {
        let a0: u8 = 1;
        let a1: u16 = 1;
        let a2: u32 = 1;
        let a3: u64 = 1;
        let a4: u128 = 1;
        let a5: i8 = 1;
        let a6: i16 = 1;
        let a7: i32 = 1;
        let a8: i64 = 1;
        let a9: i128 = 1;
        let a10: scalar = 1;
        let a11: group = 0;
        let a12: field = 1;
        let a13: bool = true;

        // Bitwise AND
        let b0 = a0 & 1;
        let b1 = a1 & 1u16;
        let b2 = 1 & a2;
        let b3 = 1u64 & a3;
        let b4: u128 = a4 & 1;
        let b5: i8 = a5 & 1i8;
        let b6: i16 = 1 & a6;
        let b7: i32 = 1i32 & a7;
        let b8: i64 = 1 & 1;
        let b9: i128 = 1i128 & 1i128;
        let b10: bool = true & a13;

        // Bitwise OR
        let c0 = a0 | 1;
        let c1 = a1 | 1u16;
        let c2 = 1 | a2;
        let c3 = 1u64 | a3;
        let c4: u128 = a4 | 1;
        let c5: i8 = a5 | 1i8;
        let c6: i16 = 1 | a6;
        let c7: i32 = 1i32 | a7;
        let c8: i64 = 1 | 1;
        let c9: i128 = 1i128 | 1i128;
        let c10: bool = true | a13;

        // Bitwise XOR
        let d0 = a0 ^ 1;
        let d1 = a1 ^ 1u16;
        let d2 = 1 ^ a2;
        let d3 = 1u64 ^ a3;
        let d4: u128 = a4 ^ 1;
        let d5: i8 = a5 ^ 1i8;
        let d6: i16 = 1 ^ a6;
        let d7: i32 = 1i32 ^ a7;
        let d8: i64 = 1 ^ 1;
        let d9: i128 = 1i128 ^ 1i128;
        let d10: bool = true ^ a13;

        // Addition
        let e0 = a0 + 1;
        let e1 = a1 + 1u16;
        let e2 = 1 + a2;
        let e3 = 1u64 + a3;
        let e4: u128 = a4 + 1;
        let e5: i8 = a5 + 1i8;
        let e6: i16 = 1 + a6;
        let e7: i32 = 1i32 + a7;
        let e8: i64 = 1 + 1;
        let e9: i128 = 1i128 + 1i128;
        let e10 = a10 + 1;
        let e11 = 0 + a11;
        let e12 = 1 + a12;

        // Subtraction
        let f0 = a0 - 1;
        let f1 = a1 - 1u16;
        let f2 = 1 - a2;
        let f3 = 1u64 - a3;
        let f4: u128 = a4 - 1;
        let f5: i8 = a5 - 1i8;
        let f6: i16 = 1 - a6;
        let f7: i32 = 1i32 - a7;
        let f8: i64 = 1 - 1;
        let f9: i128 = 1i128 - 1i128;
        let f10 = a11 - 0;
        let f11 = 1 - a12;

        // Multiplication
        let g0 = a0 * 1;
        let g1 = a1 * 1u16;
        let g2 = 1 * a2;
        let g3 = 1u64 * a3;
        let g4: u128 = a4 * 1;
        let g5: i8 = a5 * 1i8;
        let g6: i16 = 1 * a6;
        let g7: i32 = 1i32 * a7;
        let g8: i64 = 1 * 1;
        let g9: i128 = 1i128 * 1i128;
        let g10 = a11 * 1; // infer 1 as a scalar
        let g11 = 1 * a11; // infer 1 as a scalar
        let g12 = a10 * 2; // infer 2 as a group
        let g13 = 2 * a10; // infer 2 as a group
        let g14 = a12 * 1;

        // Division
        let h0 = a0 / 1;
        let h1 = a1 / 1u16;
        let h2 = 1 / a2;
        let h3 = 1u64 / a3;
        let h4: u128 = a4 / 1;
        let h5: i8 = a5 / 1i8;
        let h6: i16 = 1 / a6;
        let h7: i32 = 1i32 / a7;
        let h8: i64 = 1 / 1;
        let h9: i128 = 1i128 / 1i128;
        let h10 = a12 / 1;

        // Rem
        let i0 = a0 % 1;
        let i1 = a1 % 1u16;
        let i2 = 1 % a2;
        let i3 = 1u64 % a3;
        let i4: u128 = a4 % 1;
        let i5: i8 = a5 % 1i8;
        let i6: i16 = 1 % a6;
        let i7: i32 = 1i32 % a7;
        let i8_: i64 = 1 % 1;
        let i9: i128 = 1i128 % 1i128;

        // Rem Wrapped
        let j0 = a0.rem_wrapped(1);
        let j1 = a1.rem_wrapped(1u16);
        let j2 = 1.rem_wrapped(a2);
        let j3 = 1u64.rem_wrapped(a3);
        let j4: u128 = a4.rem_wrapped(1);
        let j5: i8 = a5.rem_wrapped(1i8);
        let j6: i16 = 1.rem_wrapped(a6);
        let j7: i32 = 1i32.rem_wrapped(a7);
        let j8: i64 = 1.rem_wrapped(1);
        let j9: i128 = 1i128.rem_wrapped(1i128);

        // mod operator (unsigned integers only)
        let k0 = a0.mod(1);
        let k1 = a1.mod(1u16);
        let k2 = 1.mod(a2);
        let k3 = 1u64.mod(a3);
        let k4: u128 = a4.mod(1);
        let k5: u8 = a0.mod(1u8);
        let k6: u16 = 1.mod(a1);
        let k7: u32 = 1u32.mod(a2);
        let k8: u64 = 1.mod(1);
        let k9: u128 = 1u128.mod(1u128);

        // Pow
        let l0 = a0 ** 1u8;
        let l1 = a1 ** 1u16;
        let l2: u8 = 1 ** a2;
        let l3 = 1u64 ** a2;
        let l4: u128 = a4 ** 1u32;
        let l5: i8 = a5 ** 1u8;
        let l6: i16 = 1 ** a1;
        let l7: i32 = 1i32 ** a2;
        let l8: i64 = 1 ** 1u8;
        let l9: i128 = 1i128 ** 1u32;
        let l10: field = 1 ** 1; // infer the 1s as `field`
        let l11 = 1field ** 1; // infer 1 as `field`
        let l12 = 1 ** 1field; // infer 1 as `field`

        // Pow Wrapped
        let m0 = a0.pow_wrapped(1u8);
        let m1 = a1.pow_wrapped(1u16);
        let m2: u8 = 1.pow_wrapped(a2);
        let m3 = 1u64.pow_wrapped(a2);
        let m4: u128 = a4.pow_wrapped(1u32);
        let m5: i8 = a5.pow_wrapped(1u8);
        let m6: i16 = 1.pow_wrapped(a1);
        let m7: i32 = 1i32.pow_wrapped(a2);
        let m8: i64 = 1.pow_wrapped(1u8);
        let m9: i128 = 1i128.pow_wrapped(1u32);

        // Eq, Neq
        let n0 = a0 == 1;
        let n1 = a1 != 1;
        let n2 = 1 == a2;
        let n3 = 1 != a3;
        let n4 = a4 == 1;
        let n5 = a5 != 1;
        let n6 = 1 == a6;
        let n7 = 1 != a7;
        let n8 = 1 == a8;
        let n9 = a9 != 1;
        let n10 = 1 == a10;
        let n11 = 0 != a11;
        let n12 = 1 == a12;

        // Lt, Gt, Lte, Gte
        let o0 = a0 > 1;
        let o1 = a1 < 1;
        let o2 = 1 <= a2;
        let o3 = 1 >= a3;
        let o4 = a4 > 1;
        let o5 = a5 < 1;
        let o6 = 1 <= a6;
        let o7 = 1 >= a7;
        let o8 = 1 > a8;
        let o9 = a9 < 1;
        let o10 = 1 <= a10;
        let o12 = 1 > a12;

        // add_wrapped (integers only)
        let p0 = a0.add_wrapped(1);
        let p1 = a1.add_wrapped(1u16);
        let p2 = 1.add_wrapped(a2);
        let p3 = 1u64.add_wrapped(a3);
        let p4: u128 = a4.add_wrapped(1);
        let p5: i8 = a5.add_wrapped(1i8);
        let p6: i16 = 1.add_wrapped(a6);
        let p7: i32 = 1i32.add_wrapped(a7);
        let p8: i64 = 1.add_wrapped(1);
        let p9: i128 = 1i128.add_wrapped(1i128);

        // sub_wrapped (integers only)
        let q0 = a0.sub_wrapped(1);
        let q1 = a1.sub_wrapped(1u16);
        let q2 = 1.sub_wrapped(a2);
        let q3 = 1u64.sub_wrapped(a3);
        let q4: u128 = a4.sub_wrapped(1);
        let q5: i8 = a5.sub_wrapped(1i8);
        let q6: i16 = 1.sub_wrapped(a6);
        let q7: i32 = 1i32.sub_wrapped(a7);
        let q8: i64 = 1.sub_wrapped(1);
        let q9: i128 = 1i128.sub_wrapped(1i128);

        // mul_wrapped (integers only)
        let r0 = a0.mul_wrapped(1);
        let r1 = a1.mul_wrapped(1u16);
        let r2 = 1.mul_wrapped(a2);
        let r3 = 1u64.mul_wrapped(a3);
        let r4: u128 = a4.mul_wrapped(1);
        let r5: i8 = a5.mul_wrapped(1i8);
        let r6: i16 = 1.mul_wrapped(a6);
        let r7: i32 = 1i32.mul_wrapped(a7);
        let r8: i64 = 1.mul_wrapped(1);
        let r9: i128 = 1i128.mul_wrapped(1i128);

        // div_wrapped (integers only)
        let s0 = a0.div_wrapped(1);
        let s1 = a1.div_wrapped(1u16);
        let s2 = 1.div_wrapped(a2);
        let s3 = 1u64.div_wrapped(a3);
        let s4: u128 = a4.div_wrapped(1);
        let s5: i8 = a5.div_wrapped(1i8);
        let s6: i16 = 1.div_wrapped(a6);
        let s7: i32 = 1i32.div_wrapped(a7);
        let s8: i64 = 1.div_wrapped(1);
        let s9: i128 = 1i128.div_wrapped(1i128);

        // Bitwise left shift `<<`
        let t0 = a0 << 1u8;
        let t1 = a1 << 1u16;
        let t2: u8 = 1 << a2;
        let t3 = 1u64 << a2;
        let t4: u128 = a4 << 1u32;
        let t5: i8 = a5 << 1u8;
        let t6: i16 = 1 << a1;
        let t7: i32 = 1i32 << a2;
        let t8: i64 = 1 << 1u8;
        let t9: i128 = 1i128 << 1u32;

        // shl_wrapped
        let u0 = a0.shl_wrapped(1u8);
        let u1 = a1.shl_wrapped(1u16);
        let u2: u8 = 1.shl_wrapped(a2);
        let u3 = 1u64.shl_wrapped(a2);
        let u4: u128 = a4.shl_wrapped(1u32);
        let u5: i8 = a5.shl_wrapped(1u8);
        let u6: i16 = 1.shl_wrapped(a1);
        let u7: i32 = 1i32.shl_wrapped(a2);
        let u8_: i64 = 1.shl_wrapped(1u8);
        let u9: i128 = 1i128.shl_wrapped(1u32);

        // Bitwise right shift `>>`
        let v0 = a0 >> 1u8;
        let v1 = a1 >> 1u16;
        let v2: u8 = 1 >> a2;
        let v3 = 1u64 >> a2;
        let v4: u128 = a4 >> 1u32;
        let v5: i8 = a5 >> 1u8;
        let v6: i16 = 1 >> a1;
        let v7: i32 = 1i32 >> a2;
        let v8: i64 = 1 >> 1u8;
        let v9: i128 = 1i128 >> 1u32;

        // shr_wrapped
        let w0 = a0.shr_wrapped(1u8);
        let w1 = a1.shr_wrapped(1u16);
        let w2: u8 = 1.shr_wrapped(a2);
        let w3 = 1u64.shr_wrapped(a2);
        let w4: u128 = a4.shr_wrapped(1u32);
        let w5: i8 = a5.shr_wrapped(1u8);
        let w6: i16 = 1.shr_wrapped(a1);
        let w7: i32 = 1i32.shr_wrapped(a2);
        let w8: i64 = 1.shr_wrapped(1u8);
        let w9: i128 = 1i128.shr_wrapped(1u32);

        let z0: i8 = -1.abs();
        let z1: i16 = -1.abs_wrapped();
        let z2: field = 0.double();
        let z3: group = 0.double();
        let z4 = 2.inv();
        let z5: field = 2.inv();
        let z6: i32  = -2;
        let z7: i64  = !2;
        let z8 = 2.square(); // 2 must be field
        let z9: field = 2.square(); // 2 must be field
        let z10 = 2.square_root(); // 2 must be field
        let z11: field = 2.square_root(); // 2 must be field
        let z12 = 2.to_x_coordinate(); // 2 must be field
        let z13: field = 2.to_x_coordinate(); // 2 must be field
        let z14 = 2.to_y_coordinate(); // 2 must be group
        let z15: field = 2.to_y_coordinate(); // 2 must be group
    }
}
