program test.aleo {
    fn main(a: i8, b: i8) -> bool {
        // unary
        let _c: i8 = a.abs();
        let _d: i8 = a.abs_wrapped();
        let _g: i8 = a.neg();
        let _h: i8 = a.not();

        // binary
        let _j: i8 = a.add(b);
        let _k: i8 = a.add_wrapped(b);
        let _l: i8 = a.and(b);
        let _m: i8 = a.div(b);
        let _n: i8 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: i8 = a.mul(b);
        let _u: i8 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: i8 = a.or(b);
        let _z: i8 = a.pow(2u8);
        let _aa: i8 = a.pow(2u16);
        let _ab: i8 = a.pow(2u32);
        let _ac: i8 = a.pow_wrapped(2u8);
        let _ad: i8 = a.pow_wrapped(2u16);
        let _ae: i8 = a.pow_wrapped(2u32);
        let _af: i8 = a.shl(2u8);
        let _ag: i8 = a.shl(2u16);
        let _ah: i8 = a.shl(2u32);
        let _ai: i8 = a.shl_wrapped(2u8);
        let _aj: i8 = a.shl_wrapped(2u16);
        let _ak: i8 = a.shl_wrapped(2u32);
        let _al: i8 = a.shr(2u8);
        let _am: i8 = a.shr(2u16);
        let _an: i8 = a.shr(2u32);
        let _ao: i8 = a.shr_wrapped(2u8);
        let _ap: i8 = a.shr_wrapped(2u16);
        let _aq: i8 = a.shr_wrapped(2u32);
        let _ar: i8 = a.xor(b);
        let _at: i8 = a.rem(b);
        let _au: i8 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
