/*
namespace: Parse
expectation: Fail
*/

circuit X {
    function x(foo: u32, &self) {
        return ();
    }
}