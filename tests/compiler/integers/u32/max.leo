/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(y: bool) -> bool {
        const a: u32 = 4294967295u32;
        return y == true;
    }
}
