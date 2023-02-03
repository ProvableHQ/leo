/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i128) -> i128 {
        let b: i128 = -170141183460469231731687303715884105727i128;
        return b - a; // -170141183460469231731687303715884105728i128
    }
}
