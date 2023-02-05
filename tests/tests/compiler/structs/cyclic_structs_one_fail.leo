/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    struct Foo {
        foo: Foo,
    }
}
