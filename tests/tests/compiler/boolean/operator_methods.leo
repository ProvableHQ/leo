/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: bool, b: bool) -> bool {
        // unary
        let h: bool = a.not();
    
        // binary
        let l: bool = a.and(b);
        let o: bool = a.eq(b);
        let v: bool = a.nand(b);
        let w: bool = a.neq(b);
        let x: bool = a.nor(b);
        let y: bool = a.or(b);
        let ar: bool = a.xor(b);
    
        return h;
    }}
