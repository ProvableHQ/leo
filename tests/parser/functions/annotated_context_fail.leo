/*
namespace: Parse
expectation: Fail
*/

@context
function f() {
    return ();
}

@context // recovery witness
function g() {
    return ();
}
