/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/lt.in
*/

program test.aleo {    
    transition main(a: u32, b: u32, c: bool) -> bool{
        return a < b == c;
    }
}
