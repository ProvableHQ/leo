program test.aleo {
    fn main(a: u128, b: u128) -> bool {
        // unary
        let _h: u128 = a.not();

        // binary
        let _j: u128 = a.add(b);
        let _k: u128 = a.add_wrapped(b);
        let _l: u128 = a.and(b);
        let _m: u128 = a.div(b);
        let _n: u128 = a.div_wrapped(b);
        let _o: bool = a.eq(b);
        let _p: bool = a.gte(b);
        let _q: bool = a.gt(b);
        let _r: bool = a.lte(b);
        let _s: bool = a.lt(b);
        let _t: u128 = a.mul(b);
        let _u: u128 = a.mul_wrapped(b);
        let _w: bool = a.neq(b);
        let _y: u128 = a.or(b);
        let _z: u128 = a.pow(2u8);
        let _aa: u128 = a.pow(2u16);
        let _ab: u128 = a.pow(2u32);
        let _ac: u128 = a.pow_wrapped(2u8);
        let _ad: u128 = a.pow_wrapped(2u16);
        let _ae: u128 = a.pow_wrapped(2u32);
        let _af: u128 = a.shl(2u8);
        let _ag: u128 = a.shl(2u16);
        let _ah: u128 = a.shl(2u32);
        let _ai: u128 = a.shl_wrapped(2u8);
        let _aj: u128 = a.shl_wrapped(2u16);
        let _ak: u128 = a.shl_wrapped(2u32);
        let _al: u128 = a.shr(2u8);
        let _am: u128 = a.shr(2u16);
        let _an: u128 = a.shr(2u32);
        let _ao: u128 = a.shr_wrapped(2u8);
        let _ap: u128 = a.shr_wrapped(2u16);
        let _aq: u128 = a.shr_wrapped(2u32);
        let _ar: u128 = a.xor(b);
        let _at: u128 = a.mod(b);
        let _au: u128 = a.rem(b);
        let _av: u128 = a.rem_wrapped(b);

        return a == b;
    }

    @noupgrade
    constructor() {}
}
