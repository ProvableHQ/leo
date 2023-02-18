/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        let a: i128 = 0i128;
    
        return (-a == 0i128) == y;
    }
}
