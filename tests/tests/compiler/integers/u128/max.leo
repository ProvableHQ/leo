/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: u128 = 340282366920938463463374607431768211455u128;
        return y == true;
    }
}
