/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i128, b: i128) -> bool {
        // unary
        let c: i128 = a.abs();
        let d: i128 = a.abs_wrapped();
        let g: i128 = a.neg();
        let h: i128 = a.not();
    
        // binary
        let j: i128 = a.add(b);
        let k: i128 = a.add_wrapped(b);
        let l: i128 = a.and(b);
        let m: i128 = a.div(b);
        let n: i128 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: i128 = a.mul(b);
        let u: i128 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: i128 = a.or(b);
        let z: i128 = a.pow(2u8);
        let aa: i128 = a.pow(2u16);
        let ab: i128 = a.pow(2u32);
        let ac: i128 = a.pow_wrapped(2u8);
        let ad: i128 = a.pow_wrapped(2u16);
        let ae: i128 = a.pow_wrapped(2u32);
        let af: i128 = a.shl(2u8);
        let ag: i128 = a.shl(2u16);
        let ah: i128 = a.shl(2u32);
        let ai: i128 = a.shl_wrapped(2u8);
        let aj: i128 = a.shl_wrapped(2u16);
        let ak: i128 = a.shl_wrapped(2u32);
        let al: i128 = a.shr(2u8);
        let am: i128 = a.shr(2u16);
        let an: i128 = a.shr(2u32);
        let ao: i128 = a.shr_wrapped(2u8);
        let ap: i128 = a.shr_wrapped(2u16);
        let aq: i128 = a.shr_wrapped(2u32);
        let ar: i128 = a.xor(b);
        let at: i128 = a.rem(b);
        let au: i128 = a.rem_wrapped(b);
    
        return a == b;
    }}
