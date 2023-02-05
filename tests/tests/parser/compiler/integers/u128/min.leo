/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool{
        const a: u128 = 0u128;
        return y == true;
    }
}
