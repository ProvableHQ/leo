/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping values: field => field;

    transition foo() {
        return then finalize();
    }
    
    finalize foo() {
        let a: field = ChaCha::rand_field(1field);
        let b: field = ChaCha::rand_field(1field, 2field);
        values.set(a, b);
    }
}
