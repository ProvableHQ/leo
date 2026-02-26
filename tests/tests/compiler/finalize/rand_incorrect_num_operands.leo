
program test.aleo {

    mapping values: field => field;

    fn foo() -> Final {
        return final { finalize_foo(); };
    }
}

final fn finalize_foo() {
    let a: field = ChaCha::rand_field(1field);
    let b: field = ChaCha::rand_field(1field, 2field);
    values.set(a, b);
}
