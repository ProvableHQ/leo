/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i8, b: i8) -> bool {
        // unary
        let c: i8 = a.abs();
        let d: i8 = a.abs_wrapped();
        let g: i8 = a.neg();
        let h: i8 = a.not();
    
        // binary
        let j: i8 = a.add(b);
        let k: i8 = a.add_wrapped(b);
        let l: i8 = a.and(b);
        let m: i8 = a.div(b);
        let n: i8 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: i8 = a.mul(b);
        let u: i8 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: i8 = a.or(b);
        let z: i8 = a.pow(2u8);
        let aa: i8 = a.pow(2u16);
        let ab: i8 = a.pow(2u32);
        let ac: i8 = a.pow_wrapped(2u8);
        let ad: i8 = a.pow_wrapped(2u16);
        let ae: i8 = a.pow_wrapped(2u32);
        let af: i8 = a.shl(2u8);
        let ag: i8 = a.shl(2u16);
        let ah: i8 = a.shl(2u32);
        let ai: i8 = a.shl_wrapped(2u8);
        let aj: i8 = a.shl_wrapped(2u16);
        let ak: i8 = a.shl_wrapped(2u32);
        let al: i8 = a.shr(2u8);
        let am: i8 = a.shr(2u16);
        let an: i8 = a.shr(2u32);
        let ao: i8 = a.shr_wrapped(2u8);
        let ap: i8 = a.shr_wrapped(2u16);
        let aq: i8 = a.shr_wrapped(2u32);
        let ar: i8 = a.xor(b);
        let at: i8 = a.rem(b);
        let au: i8 = a.rem_wrapped(b);
    
        return a == b;
    }}
