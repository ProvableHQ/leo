program test.aleo {
    fn main(a: u8, b: u8) -> bool {
        // unary
        let _h: u8 = a.not();

        // binary
        let _j: u8 = a.add(b);
        let _k: u8 = a.add_wrapped(b);
        let _l: u8 = a.and(b);
        let _m: u8 = a.div(b);
        let _n: u8 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: u8 = a.mul(b);
        let _u: u8 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: u8 = a.or(b);
        let _z: u8 = a.pow(b);
        let _aa: u8 = a.pow(2u16);
        let _ab: u8 = a.pow(2u32);
        let _ac: u8 = a.pow_wrapped(b);
        let _ad: u8 = a.pow_wrapped(2u16);
        let _ae: u8 = a.pow_wrapped(2u32);
        let _af: u8 = a.shl(b);
        let _ag: u8 = a.shl(2u16);
        let _ah: u8 = a.shl(2u32);
        let _ai: u8 = a.shl_wrapped(b);
        let _aj: u8 = a.shl_wrapped(2u16);
        let _ak: u8 = a.shl_wrapped(2u32);
        let _al: u8 = a.shr(b);
        let _am: u8 = a.shr(2u16);
        let _an: u8 = a.shr(2u32);
        let _ao: u8 = a.shr_wrapped(b);
        let _ap: u8 = a.shr_wrapped(2u16);
        let _aq: u8 = a.shr_wrapped(2u32);
        let _ar: u8 = a.xor(b);
        let _at: u8 = a.mod(b);
        let _au: u8 = a.rem(b);
        let _av: u8 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
