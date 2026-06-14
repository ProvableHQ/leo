program test.aleo {
    fn main(a: u16, b: u16) -> bool {
        // unary
        let _h: u16 = a.not();

        // binary
        let _j: u16 = a.add(b);
        let _k: u16 = a.add_wrapped(b);
        let _l: u16 = a.and(b);
        let _m: u16 = a.div(b);
        let _n: u16 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: u16 = a.mul(b);
        let _u: u16 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: u16 = a.or(b);
        let _z: u16 = a.pow(2u8);
        let _aa: u16 = a.pow(b);
        let _ab: u16 = a.pow(2u32);
        let _ac: u16 = a.pow_wrapped(2u8);
        let _ad: u16 = a.pow_wrapped(b);
        let _ae: u16 = a.pow_wrapped(2u32);
        let _af: u16 = a.shl(2u8);
        let _ag: u16 = a.shl(b);
        let _ah: u16 = a.shl(2u32);
        let _ai: u16 = a.shl_wrapped(2u8);
        let _aj: u16 = a.shl_wrapped(b);
        let _ak: u16 = a.shl_wrapped(2u32);
        let _al: u16 = a.shr(2u8);
        let _am: u16 = a.shr(b);
        let _an: u16 = a.shr(2u32);
        let _ao: u16 = a.shr_wrapped(2u8);
        let _ap: u16 = a.shr_wrapped(b);
        let _aq: u16 = a.shr_wrapped(2u32);
        let _ar: u16 = a.xor(b);
        let _at: u16 = a.mod(b);
        let _au: u16 = a.rem(b);
        let _av: u16 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
