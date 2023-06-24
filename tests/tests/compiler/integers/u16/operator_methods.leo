/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    
    transition main(a: u16, b: u16) -> bool {
        // unary
        let h: u16 = a.not();
    
        // binary
        let j: u16 = a.add(b);
        let k: u16 = a.add_wrapped(b);
        let l: u16 = a.and(b);
        let m: u16 = a.div(b);
        let n: u16 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: u16 = a.mul(b);
        let u: u16 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: u16 = a.or(b);
        let z: u16 = a.pow(2u8);
        let aa: u16 = a.pow(b);
        let ab: u16 = a.pow(2u32);
        let ac: u16 = a.pow_wrapped(2u8);
        let ad: u16 = a.pow_wrapped(b);
        let ae: u16 = a.pow_wrapped(2u32);
        let af: u16 = a.shl(2u8);
        let ag: u16 = a.shl(b);
        let ah: u16 = a.shl(2u32);
        let ai: u16 = a.shl_wrapped(2u8);
        let aj: u16 = a.shl_wrapped(b);
        let ak: u16 = a.shl_wrapped(2u32);
        let al: u16 = a.shr(2u8);
        let am: u16 = a.shr(b);
        let an: u16 = a.shr(2u32);
        let ao: u16 = a.shr_wrapped(2u8);
        let ap: u16 = a.shr_wrapped(b);
        let aq: u16 = a.shr_wrapped(2u32);
        let ar: u16 = a.xor(b);
        let at: u16 = a.mod(b);
        let au: u16 = a.rem(b);
        let av: u16 = a.rem_wrapped(b);
    
        return a == b;
    }}
