/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i8, b: i8, c: i8) -> bool {
        return a + b + 1_1i8 + 1______1i8 == c + 1i8;
    }
}
