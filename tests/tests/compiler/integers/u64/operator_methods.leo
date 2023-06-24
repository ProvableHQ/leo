/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u64, b: u64) -> bool {
        // unary
        let h: u64 = a.not();
    
        // binary
        let j: u64 = a.add(b);
        let k: u64 = a.add_wrapped(b);
        let l: u64 = a.and(b);
        let m: u64 = a.div(b);
        let n: u64 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: u64 = a.mul(b);
        let u: u64 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: u64 = a.or(b);
        let z: u64 = a.pow(2u8);
        let aa: u64 = a.pow(2u16);
        let ab: u64 = a.pow(2u32);
        let ac: u64 = a.pow_wrapped(2u8);
        let ad: u64 = a.pow_wrapped(2u16);
        let ae: u64 = a.pow_wrapped(2u32);
        let af: u64 = a.shl(2u8);
        let ag: u64 = a.shl(2u16);
        let ah: u64 = a.shl(2u32);
        let ai: u64 = a.shl_wrapped(2u8);
        let aj: u64 = a.shl_wrapped(2u16);
        let ak: u64 = a.shl_wrapped(2u32);
        let al: u64 = a.shr(2u8);
        let am: u64 = a.shr(2u16);
        let an: u64 = a.shr(2u32);
        let ao: u64 = a.shr_wrapped(2u8);
        let ap: u64 = a.shr_wrapped(2u16);
        let aq: u64 = a.shr_wrapped(2u32);
        let ar: u64 = a.xor(b);
        let at: u64 = a.mod(b);
        let au: u64 = a.rem(b);
        let av: u64 = a.rem_wrapped(b);
    
        return a == b;
    }}
