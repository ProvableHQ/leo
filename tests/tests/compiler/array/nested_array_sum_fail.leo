/*
namespace: Compile
expectation: Fail
*/


program test.aleo {
    transition foo(a: [u32; 4]) {
        let sum: u32 = 0u32;
        for i: u32 in 0u32..4u32 {
            for j: u32 in 0u32..4u32 {
                sum = sum + a[i][j];
            }
        }
    }
}
