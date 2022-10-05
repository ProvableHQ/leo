/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/ge.in
*/

program test.aleo {    
    transition main(a: i64, b: i64, c: bool) -> bool {
        return a >= b == c;
    }
}
