/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping values: scalar => group;

    transition foo() {
        return then finalize();
    }

    finalize foo() {
        let a: scalar = ChaCha::rand_field();
        let b: group = ChaCha::rand_field();
        values.set(a, b);
    }
}
