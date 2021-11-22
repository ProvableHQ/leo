/*
namespace: Parse
expectation: Fail
*/

struct A {
    function a() {}
    const function z() {}

    foo: u8,

    static const BAR: u8 = 0u8;
}

struct B {
    foo: u8,

    static const BAR: u8 = 0u8;
}
