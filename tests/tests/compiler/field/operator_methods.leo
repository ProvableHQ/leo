/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: field, b: field) -> bool {
       // unary
       let f: field = a.inv();
       let g: field = a.neg();
       let i: field = a.square();
       let j: field = a.square_root();
    
       // binary
       let k: field = a.add(b);
       let m: field = a.div(b);
       let o: bool = a.eq(b);
       let p: bool = a.gte(b);
       let q: bool = a.gt(b);
       let r: bool = a.lte(b);
       let s: bool = a.lt(b);
       let t: field = a.mul(b);
       let w: bool = a.neq(b);
       let z: field = a.pow(b);
    
       return w;
    }}
