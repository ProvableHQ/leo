program test.aleo {
    fn main(a: i32, b: i32) -> bool {
        // unary
        let _c: i32 = a.abs();
        let _d: i32 = a.abs_wrapped();
        let _g: i32 = a.neg();
        let _h: i32 = a.not();

        // binary
        let _j: i32 = a.add(b);
        let _k: i32 = a.add_wrapped(b);
        let _l: i32 = a.and(b);
        let _m: i32 = a.div(b);
        let _n: i32 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: i32 = a.mul(b);
        let _u: i32 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: i32 = a.or(b);
        let _z: i32 = a.pow(2u8);
        let _aa: i32 = a.pow(2u16);
        let _ab: i32 = a.pow(2u32);
        let _ac: i32 = a.pow_wrapped(2u8);
        let _ad: i32 = a.pow_wrapped(2u16);
        let _ae: i32 = a.pow_wrapped(2u32);
        let _af: i32 = a.shl(2u8);
        let _ag: i32 = a.shl(2u16);
        let _ah: i32 = a.shl(2u32);
        let _ai: i32 = a.shl_wrapped(2u8);
        let _aj: i32 = a.shl_wrapped(2u16);
        let _ak: i32 = a.shl_wrapped(2u32);
        let _al: i32 = a.shr(2u8);
        let _am: i32 = a.shr(2u16);
        let _an: i32 = a.shr(2u32);
        let _ao: i32 = a.shr_wrapped(2u8);
        let _ap: i32 = a.shr_wrapped(2u16);
        let _aq: i32 = a.shr_wrapped(2u32);
        let _ar: i32 = a.xor(b);
        let _at: i32 = a.rem(b);
        let _au: i32 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
