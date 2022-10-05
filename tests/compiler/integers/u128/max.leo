/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: u128 = 340282366920938463463374607431768211455u128;
        return y == true;
    }
}
