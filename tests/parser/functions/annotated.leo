/*
namespace: Parse
expectation: Pass
*/

@test
function a() {
    return ();
}

@test(test)
function b() {
    return ();
}

@test(test,)
function c() {
    return ();
}

@test()
function d() {
    return ();
}
