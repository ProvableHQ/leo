/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(a: bool, b: bool) -> (bool, bool) {
        let t: (bool, u64) = (a, b); // We should expect a boolean, not a u64.
    
        return (t.0, t.1);
    }
}
