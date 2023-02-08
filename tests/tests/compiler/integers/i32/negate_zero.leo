/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: i32 = 0i32;
    
        return (-a == 0i32) == y;
    }
}
