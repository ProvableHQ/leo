/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/le.in
*/

program test.aleo {    
    transition main(a: u128, b: u128, c: bool) -> bool {
        return a <= b == c;
    }
}
