program test.aleo {
    fn main(a: u32, b: u32) -> bool {
        // unary
        let _h: u32 = a.not();

        // binary
        let _j: u32 = a.add(b);
        let _k: u32 = a.add_wrapped(b);
        let _l: u32 = a.and(b);
        let _m: u32 = a.div(b);
        let _n: u32 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: u32 = a.mul(b);
        let _u: u32 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: u32 = a.or(b);
        let _z: u32 = a.pow(2u8);
        let _aa: u32 = a.pow(2u16);
        let _ab: u32 = a.pow(b);
        let _ac: u32 = a.pow_wrapped(2u8);
        let _ad: u32 = a.pow_wrapped(2u16);
        let _ae: u32 = a.pow_wrapped(b);
        let _af: u32 = a.shl(2u8);
        let _ag: u32 = a.shl(2u16);
        let _ah: u32 = a.shl(b);
        let _ai: u32 = a.shl_wrapped(2u8);
        let _aj: u32 = a.shl_wrapped(2u16);
        let _ak: u32 = a.shl_wrapped(b);
        let _al: u32 = a.shr(2u8);
        let _am: u32 = a.shr(2u16);
        let _an: u32 = a.shr(b);
        let _ao: u32 = a.shr_wrapped(2u8);
        let _ap: u32 = a.shr_wrapped(2u16);
        let _aq: u32 = a.shr_wrapped(b);
        let _ar: u32 = a.xor(b);
        let _at: u32 = a.mod(b);
        let _au: u32 = a.rem(b);
        let _av: u32 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
