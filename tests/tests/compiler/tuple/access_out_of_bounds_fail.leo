/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: bool, b: bool) -> (bool, bool) {
        let t: (bool, bool) = (a, b);
    
        return (t.0, t.2); // Index `t.2` is out of bounds.
    }
}
