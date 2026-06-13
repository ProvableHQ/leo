program test.aleo {
    fn main(a: i128, b: i128) -> bool {
        // unary
        let _c: i128 = a.abs();
        let _d: i128 = a.abs_wrapped();
        let _g: i128 = a.neg();
        let _h: i128 = a.not();

        // binary
        let _j: i128 = a.add(b);
        let _k: i128 = a.add_wrapped(b);
        let _l: i128 = a.and(b);
        let _m: i128 = a.div(b);
        let _n: i128 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: i128 = a.mul(b);
        let _u: i128 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: i128 = a.or(b);
        let _z: i128 = a.pow(2u8);
        let _aa: i128 = a.pow(2u16);
        let _ab: i128 = a.pow(2u32);
        let _ac: i128 = a.pow_wrapped(2u8);
        let _ad: i128 = a.pow_wrapped(2u16);
        let _ae: i128 = a.pow_wrapped(2u32);
        let _af: i128 = a.shl(2u8);
        let _ag: i128 = a.shl(2u16);
        let _ah: i128 = a.shl(2u32);
        let _ai: i128 = a.shl_wrapped(2u8);
        let _aj: i128 = a.shl_wrapped(2u16);
        let _ak: i128 = a.shl_wrapped(2u32);
        let _al: i128 = a.shr(2u8);
        let _am: i128 = a.shr(2u16);
        let _an: i128 = a.shr(2u32);
        let _ao: i128 = a.shr_wrapped(2u8);
        let _ap: i128 = a.shr_wrapped(2u16);
        let _aq: i128 = a.shr_wrapped(2u32);
        let _ar: i128 = a.xor(b);
        let _at: i128 = a.rem(b);
        let _au: i128 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
