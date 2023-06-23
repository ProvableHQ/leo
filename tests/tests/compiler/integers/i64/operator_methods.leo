/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i64, b: i64) -> bool {
        // unary
        let c: i64 = a.abs();
        let d: i64 = a.abs_wrapped();
        let g: i64 = a.neg();
        let h: i64 = a.not();
    
        // binary
        let j: i64 = a.add(b);
        let k: i64 = a.add_wrapped(b);
        let l: i64 = a.and(b);
        let m: i64 = a.div(b);
        let n: i64 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: i64 = a.mul(b);
        let u: i64 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: i64 = a.or(b);
        let z: i64 = a.pow(2u8);
        let aa: i64 = a.pow(2u16);
        let ab: i64 = a.pow(2u32);
        let ac: i64 = a.pow_wrapped(2u8);
        let ad: i64 = a.pow_wrapped(2u16);
        let ae: i64 = a.pow_wrapped(2u32);
        let af: i64 = a.shl(2u8);
        let ag: i64 = a.shl(2u16);
        let ah: i64 = a.shl(2u32);
        let ai: i64 = a.shl_wrapped(2u8);
        let aj: i64 = a.shl_wrapped(2u16);
        let ak: i64 = a.shl_wrapped(2u32);
        let al: i64 = a.shr(2u8);
        let am: i64 = a.shr(2u16);
        let an: i64 = a.shr(2u32);
        let ao: i64 = a.shr_wrapped(2u8);
        let ap: i64 = a.shr_wrapped(2u16);
        let aq: i64 = a.shr_wrapped(2u32);
        let ar: i64 = a.xor(b);
        let at: i64 = a.rem(b);
        let au: i64 = a.rem_wrapped(b);
    
        return a == b;
    }}
