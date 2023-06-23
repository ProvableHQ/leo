/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    
    transition main(a: u32, b: u32) -> bool {
        // unary
        let h: u32 = a.not();
    
        // binary
        let j: u32 = a.add(b);
        let k: u32 = a.add_wrapped(b);
        let l: u32 = a.and(b);
        let m: u32 = a.div(b);
        let n: u32 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: u32 = a.mul(b);
        let u: u32 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: u32 = a.or(b);
        let z: u32 = a.pow(2u8);
        let aa: u32 = a.pow(2u16);
        let ab: u32 = a.pow(b);
        let ac: u32 = a.pow_wrapped(2u8);
        let ad: u32 = a.pow_wrapped(2u16);
        let ae: u32 = a.pow_wrapped(b);
        let af: u32 = a.shl(2u8);
        let ag: u32 = a.shl(2u16);
        let ah: u32 = a.shl(b);
        let ai: u32 = a.shl_wrapped(2u8);
        let aj: u32 = a.shl_wrapped(2u16);
        let ak: u32 = a.shl_wrapped(b);
        let al: u32 = a.shr(2u8);
        let am: u32 = a.shr(2u16);
        let an: u32 = a.shr(b);
        let ao: u32 = a.shr_wrapped(2u8);
        let ap: u32 = a.shr_wrapped(2u16);
        let aq: u32 = a.shr_wrapped(b);
        let ar: u32 = a.xor(b);
        let at: u32 = a.mod(b);
        let au: u32 = a.rem(b);
        let av: u32 = a.rem_wrapped(b);
    
        return a == b;
    }}
