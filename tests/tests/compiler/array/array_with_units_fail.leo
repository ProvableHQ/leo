/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo() -> bool {
        let bar: [(); 2] = [(), ()];
    }
}
