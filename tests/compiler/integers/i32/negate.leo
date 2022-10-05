/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

program test.aleo {    
    transition main(a: i32, b: i32) -> bool {
        return -a == b;
    }
}
