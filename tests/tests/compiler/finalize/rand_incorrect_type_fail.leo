
program test.aleo {

    mapping values: scalar => group;

    fn foo() -> Final {
        return final { finalize_foo(); };
    }
}

final fn finalize_foo() {
    let a: scalar = ChaCha::rand_field();
    let b: group = ChaCha::rand_field();
    values.set(a, b);
}
