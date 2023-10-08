/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo() -> bool {
        let bar: [(), 2] = [(), ()];
    }
}
