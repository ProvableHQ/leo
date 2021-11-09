/*
namespace: Parse
expectation: Fail
*/

circuit A {
    function a() {}
    const function z() {}

    foo: u8,

    static const BAR: u8 = 0u8;
}

circuit B {
    foo: u8,

    static const BAR: u8 = 0u8;
}
