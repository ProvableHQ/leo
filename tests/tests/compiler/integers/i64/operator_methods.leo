program test.aleo {
    fn main(a: i64, b: i64) -> bool {
        // unary
        let _c: i64 = a.abs();
        let _d: i64 = a.abs_wrapped();
        let _g: i64 = a.neg();
        let _h: i64 = a.not();

        // binary
        let _j: i64 = a.add(b);
        let _k: i64 = a.add_wrapped(b);
        let _l: i64 = a.and(b);
        let _m: i64 = a.div(b);
        let _n: i64 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: i64 = a.mul(b);
        let _u: i64 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: i64 = a.or(b);
        let _z: i64 = a.pow(2u8);
        let _aa: i64 = a.pow(2u16);
        let _ab: i64 = a.pow(2u32);
        let _ac: i64 = a.pow_wrapped(2u8);
        let _ad: i64 = a.pow_wrapped(2u16);
        let _ae: i64 = a.pow_wrapped(2u32);
        let _af: i64 = a.shl(2u8);
        let _ag: i64 = a.shl(2u16);
        let _ah: i64 = a.shl(2u32);
        let _ai: i64 = a.shl_wrapped(2u8);
        let _aj: i64 = a.shl_wrapped(2u16);
        let _ak: i64 = a.shl_wrapped(2u32);
        let _al: i64 = a.shr(2u8);
        let _am: i64 = a.shr(2u16);
        let _an: i64 = a.shr(2u32);
        let _ao: i64 = a.shr_wrapped(2u8);
        let _ap: i64 = a.shr_wrapped(2u16);
        let _aq: i64 = a.shr_wrapped(2u32);
        let _ar: i64 = a.xor(b);
        let _at: i64 = a.rem(b);
        let _au: i64 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
