/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: group, b: group) -> bool {
        // unary
        let e: group = a.double();
        let g: group = a.neg();
    
        // binary
        let j: group = a.add(b);
        let o: bool = a.eq(b);
        let t: group = a.mul(2scalar);
        let q: group = 2scalar.mul(a);
        let w: bool = a.neq(b);
    
        return a == b;
    }}
