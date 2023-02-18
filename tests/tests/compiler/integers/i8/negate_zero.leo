/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        let a: i8 = 0i8;
    
        return (-a == 0i8) == y;
    }
}
