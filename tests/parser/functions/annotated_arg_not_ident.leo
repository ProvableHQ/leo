/*
namespace: Parse
expectation: Fail
*/

@foo(?, bar, ?)
function x() {
    return ();
}

@bar(123) // ints not vali
function x() {
    return ();
}


@context // recovery witness
function x() {
    return ();
}
