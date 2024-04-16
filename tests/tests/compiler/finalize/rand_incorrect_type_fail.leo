/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping values: scalar => group;

    async transition foo() -> Future {
        return finalize_foo();
    }

    async function finalize_foo() {
        let a: scalar = ChaCha::rand_field();
        let b: group = ChaCha::rand_field();
        values.set(a, b);
    }
}
