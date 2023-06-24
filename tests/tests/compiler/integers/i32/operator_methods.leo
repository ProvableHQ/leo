/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i32, b: i32) -> bool {
        // unary
        let c: i32 = a.abs();
        let d: i32 = a.abs_wrapped();
        let g: i32 = a.neg();
        let h: i32 = a.not();
    
        // binary
        let j: i32 = a.add(b);
        let k: i32 = a.add_wrapped(b);
        let l: i32 = a.and(b);
        let m: i32 = a.div(b);
        let n: i32 = a.div_wrapped(b);
        let o: bool = a.eq(b);
        let p: bool = a.gte(b);
        let q: bool = a.gt(b);
        let r: bool = a.lte(b);
        let s: bool = a.lt(b);
        let t: i32 = a.mul(b);
        let u: i32 = a.mul_wrapped(b);
        let w: bool = a.neq(b);
        let y: i32 = a.or(b);
        let z: i32 = a.pow(2u8);
        let aa: i32 = a.pow(2u16);
        let ab: i32 = a.pow(2u32);
        let ac: i32 = a.pow_wrapped(2u8);
        let ad: i32 = a.pow_wrapped(2u16);
        let ae: i32 = a.pow_wrapped(2u32);
        let af: i32 = a.shl(2u8);
        let ag: i32 = a.shl(2u16);
        let ah: i32 = a.shl(2u32);
        let ai: i32 = a.shl_wrapped(2u8);
        let aj: i32 = a.shl_wrapped(2u16);
        let ak: i32 = a.shl_wrapped(2u32);
        let al: i32 = a.shr(2u8);
        let am: i32 = a.shr(2u16);
        let an: i32 = a.shr(2u32);
        let ao: i32 = a.shr_wrapped(2u8);
        let ap: i32 = a.shr_wrapped(2u16);
        let aq: i32 = a.shr_wrapped(2u32);
        let ar: i32 = a.xor(b);
        let at: i32 = a.rem(b);
        let au: i32 = a.rem_wrapped(b);
    
        return a == b;
    }}
