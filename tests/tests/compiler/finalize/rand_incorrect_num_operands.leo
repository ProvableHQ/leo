
program test.aleo {

    mapping values: field => field;

    fn foo() -> Future {
        return finalize_foo();
    }
}

onchain fn finalize_foo() {
    let a: field = ChaCha::rand_field(1field);
    let b: field = ChaCha::rand_field(1field, 2field);
    values.set(a, b);
}
