/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping values: scalar => group;

    transition foo() {
        let a: scalar = ChaCha::rand_scalar();
        return then finalize(a);
    }

    finalize foo(a: scalar) {
        let b: group = ChaCha::rand_group();
        values.set(a, b);
    }
}
