/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping values: field => field;

    async transition foo() -> Future {
        return finalize_foo();
    }
    
    async function finalize_foo() {
        let a: field = ChaCha::rand_field(1field);
        let b: field = ChaCha::rand_field(1field, 2field);
        values.set(a, b);
    }
}
