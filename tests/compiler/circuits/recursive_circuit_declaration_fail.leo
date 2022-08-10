/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
  foo: Foo,
}

circuit Floo {
    bar: Bar,
}

circuit Bar {
    floo: Floo,
}