/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u8, b: u8) -> bool {
        // unary
        let h: u8 = a.not();
    
        // binary
        let j: u8 = a.add(b);
        let k: u8 = a.add_wrapped(b);
        let l: u8 = a.and(b);
        let m: u8 = a.div(b);
        let n: u8 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: u8 = a.mul(b);
        let u: u8 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: u8 = a.or(b);
        let z: u8 = a.pow(b);
        let aa: u8 = a.pow(2u16);
        let ab: u8 = a.pow(2u32);
        let ac: u8 = a.pow_wrapped(b);
        let ad: u8 = a.pow_wrapped(2u16);
        let ae: u8 = a.pow_wrapped(2u32);
        let af: u8 = a.shl(b);
        let ag: u8 = a.shl(2u16);
        let ah: u8 = a.shl(2u32);
        let ai: u8 = a.shl_wrapped(b);
        let aj: u8 = a.shl_wrapped(2u16);
        let ak: u8 = a.shl_wrapped(2u32);
        let al: u8 = a.shr(b);
        let am: u8 = a.shr(2u16);
        let an: u8 = a.shr(2u32);
        let ao: u8 = a.shr_wrapped(b);
        let ap: u8 = a.shr_wrapped(2u16);
        let aq: u8 = a.shr_wrapped(2u32);
        let ar: u8 = a.xor(b);
        let at: u8 = a.mod(b);
        let au: u8 = a.rem(b);
        let av: u8 = a.rem_wrapped(b);
    
        return a == b;
    }}
