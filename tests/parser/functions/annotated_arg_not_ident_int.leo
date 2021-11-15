/*
namespace: Parse
expectation: Fail
*/

@foo(?, bar, ?)
function x() {
    return ();
}

@context // recovery witness
function x() {
    return ();
}
