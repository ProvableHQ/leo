/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

program test.aleo {    
    transition main(a: i64, b: i64) -> bool {
        return -a == b;
    }
}
