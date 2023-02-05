/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    struct Bar {
        baz: Baz,
    }

    struct Baz {
        bar: Bar,
    }
}
