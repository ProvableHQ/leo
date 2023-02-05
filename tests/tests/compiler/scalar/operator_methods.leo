/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar, b: scalar) -> bool {
       // binary
       let j: scalar = a.add(b);
       let o: bool = a.eq(b);
       let p: bool = a.gte(b);
       let q: bool = a.gt(b);
       let r: bool = a.lte(b);
       let s: bool = a.lt(b);
       // let t: group = 2group.mul(b);
       // let u: group = a.mul(2group);
       let w: bool = a.neq(b);
    
       return a == b;
    }}
