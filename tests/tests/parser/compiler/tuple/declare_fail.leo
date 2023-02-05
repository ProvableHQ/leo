/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(a: bool, b: bool) -> (bool, bool) {
        let t: (bool, bool) = (a, 1u64); // We should be declaring to a boolean, not a u64.
    
        return (t.0, t.1);
    }
}
