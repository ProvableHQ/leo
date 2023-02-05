/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(a: bool, b: bool) -> (bool, u64) {
        let t: (bool, bool) = (a, b);
    
        return (t.0, t.1); // The second element should be type u64 as in the function declaration.
    }
}
