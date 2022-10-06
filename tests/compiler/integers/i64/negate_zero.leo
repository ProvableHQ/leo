/*
namespace: Compile
expectation: Pass
input_file:
 - ../inputs/dummy.in
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: i64 = 0i64;
    
        return (-a == 0i64) == y;
    }
}
