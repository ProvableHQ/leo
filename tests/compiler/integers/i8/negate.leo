/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

program test.aleo {    
    transition main(a: i8, b: i8) -> bool {
        return -a == b;
    }
}
