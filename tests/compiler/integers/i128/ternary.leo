/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

program test.aleo {    
    transition main(s: bool, a: i128, b: i128, c: i128) -> bool {
        let r: i128 = s ? a : b;
    
        return r == c;
    }
}
