/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        let a: i16 = 0i16;
    
        return (-a == 0i16) == y;
    }
}
