/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        let a: i64 = 0i64;
    
        return (-a == 0i64) == y;
    }
}
