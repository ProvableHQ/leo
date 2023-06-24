/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i16, b: i16) -> bool {
        // unary
        let c: i16 = a.abs();
        let d: i16 = a.abs_wrapped();
        let g: i16 = a.neg();
        let h: i16 = a.not();
    
        // binary
        let j: i16 = a.add(b);
        let k: i16 = a.add_wrapped(b);
        let l: i16 = a.and(b);
        let m: i16 = a.div(b);
        let n: i16 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: i16 = a.mul(b);
        let u: i16 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: i16 = a.or(b);
        let z: i16 = a.pow(2u8);
        let aa: i16 = a.pow(2u16);
        let ab: i16 = a.pow(2u32);
        let ac: i16 = a.pow_wrapped(2u8);
        let ad: i16 = a.pow_wrapped(2u16);
        let ae: i16 = a.pow_wrapped(2u32);
        let af: i16 = a.shl(2u8);
        let ag: i16 = a.shl(2u16);
        let ah: i16 = a.shl(2u32);
        let ai: i16 = a.shl_wrapped(2u8);
        let aj: i16 = a.shl_wrapped(2u16);
        let ak: i16 = a.shl_wrapped(2u32);
        let al: i16 = a.shr(2u8);
        let am: i16 = a.shr(2u16);
        let an: i16 = a.shr(2u32);
        let ao: i16 = a.shr_wrapped(2u8);
        let ap: i16 = a.shr_wrapped(2u16);
        let aq: i16 = a.shr_wrapped(2u32);
        let ar: i16 = a.xor(b);
        let at: i16 = a.rem(b);
        let au: i16 = a.rem_wrapped(b);
    
        return a == b;
    }}
