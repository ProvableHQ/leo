/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u128, b: u128) -> bool {
        // unary
        let h: u128 = a.not();
    
        // binary
        let j: u128 = a.add(b);
        let k: u128 = a.add_wrapped(b);
        let l: u128 = a.and(b);
        let m: u128 = a.div(b);
        let n: u128 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: u128 = a.mul(b);
        let u: u128 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: u128 = a.or(b);
        let z: u128 = a.pow(2u8);
        let aa: u128 = a.pow(2u16);
        let ab: u128 = a.pow(2u32);
        let ac: u128 = a.pow_wrapped(2u8);
        let ad: u128 = a.pow_wrapped(2u16);
        let ae: u128 = a.pow_wrapped(2u32);
        let af: u128 = a.shl(2u8);
        let ag: u128 = a.shl(2u16);
        let ah: u128 = a.shl(2u32);
        let ai: u128 = a.shl_wrapped(2u8);
        let aj: u128 = a.shl_wrapped(2u16);
        let ak: u128 = a.shl_wrapped(2u32);
        let al: u128 = a.shr(2u8);
        let am: u128 = a.shr(2u16);
        let an: u128 = a.shr(2u32);
        let ao: u128 = a.shr_wrapped(2u8);
        let ap: u128 = a.shr_wrapped(2u16);
        let aq: u128 = a.shr_wrapped(2u32);
        let ar: u128 = a.xor(b);
        let at: u128 = a.mod(b);
        let au: u128 = a.rem(b);
        let av: u128 = a.rem_wrapped(b);
    
        return a == b;
    }}
