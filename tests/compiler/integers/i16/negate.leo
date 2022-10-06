/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

program test.aleo {    
    transition main(a: i16, b: i16) -> bool {
        return -a == b;
    }
}
