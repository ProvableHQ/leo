/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping values: scalar => group;

    async transition foo() -> Future {
        let a: scalar = ChaCha::rand_scalar();
        return finalize_foo(a);
    }

    async function finalize_foo(a: scalar) {
        let b: group = ChaCha::rand_group();
        values.set(a, b);
    }
}
