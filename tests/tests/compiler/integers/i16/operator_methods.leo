program test.aleo {
    fn main(a: i16, b: i16) -> bool {
        // unary
        let _c: i16 = a.abs();
        let _d: i16 = a.abs_wrapped();
        let _g: i16 = a.neg();
        let _h: i16 = a.not();

        // binary
        let _j: i16 = a.add(b);
        let _k: i16 = a.add_wrapped(b);
        let _l: i16 = a.and(b);
        let _m: i16 = a.div(b);
        let _n: i16 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: i16 = a.mul(b);
        let _u: i16 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: i16 = a.or(b);
        let _z: i16 = a.pow(2u8);
        let _aa: i16 = a.pow(2u16);
        let _ab: i16 = a.pow(2u32);
        let _ac: i16 = a.pow_wrapped(2u8);
        let _ad: i16 = a.pow_wrapped(2u16);
        let _ae: i16 = a.pow_wrapped(2u32);
        let _af: i16 = a.shl(2u8);
        let _ag: i16 = a.shl(2u16);
        let _ah: i16 = a.shl(2u32);
        let _ai: i16 = a.shl_wrapped(2u8);
        let _aj: i16 = a.shl_wrapped(2u16);
        let _ak: i16 = a.shl_wrapped(2u32);
        let _al: i16 = a.shr(2u8);
        let _am: i16 = a.shr(2u16);
        let _an: i16 = a.shr(2u32);
        let _ao: i16 = a.shr_wrapped(2u8);
        let _ap: i16 = a.shr_wrapped(2u16);
        let _aq: i16 = a.shr_wrapped(2u32);
        let _ar: i16 = a.xor(b);
        let _at: i16 = a.rem(b);
        let _au: i16 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
