program test.aleo {
    fn main(a: u64, b: u64) -> bool {
        // unary
        let _h: u64 = a.not();

        // binary
        let _j: u64 = a.add(b);
        let _k: u64 = a.add_wrapped(b);
        let _l: u64 = a.and(b);
        let _m: u64 = a.div(b);
        let _n: u64 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: u64 = a.mul(b);
        let _u: u64 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: u64 = a.or(b);
        let _z: u64 = a.pow(2u8);
        let _aa: u64 = a.pow(2u16);
        let _ab: u64 = a.pow(2u32);
        let _ac: u64 = a.pow_wrapped(2u8);
        let _ad: u64 = a.pow_wrapped(2u16);
        let _ae: u64 = a.pow_wrapped(2u32);
        let _af: u64 = a.shl(2u8);
        let _ag: u64 = a.shl(2u16);
        let _ah: u64 = a.shl(2u32);
        let _ai: u64 = a.shl_wrapped(2u8);
        let _aj: u64 = a.shl_wrapped(2u16);
        let _ak: u64 = a.shl_wrapped(2u32);
        let _al: u64 = a.shr(2u8);
        let _am: u64 = a.shr(2u16);
        let _an: u64 = a.shr(2u32);
        let _ao: u64 = a.shr_wrapped(2u8);
        let _ap: u64 = a.shr_wrapped(2u16);
        let _aq: u64 = a.shr_wrapped(2u32);
        let _ar: u64 = a.xor(b);
        let _at: u64 = a.mod(b);
        let _au: u64 = a.rem(b);
        let _av: u64 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
