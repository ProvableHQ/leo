/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: bool, b: bool) -> (bool, bool) {
        let t: (bool, bool) = (a, b);
    
        return (t.0, t.-1); // Index `t.-1` is invalid.
    }
}
