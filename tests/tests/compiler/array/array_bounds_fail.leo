/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {
    transition foo(a: [bool; 8]) -> bool {
        let x: bool = true;
        for i: u32 in 0u32..12u32 {
            x = a[i];
        }
        return x;
    }
}
