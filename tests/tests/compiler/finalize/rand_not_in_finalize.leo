
program test.aleo {

    mapping values: scalar => group;

    fn foo() -> Final {
        let a: scalar = ChaCha::rand_scalar();
        return final { finalize_foo(a); };
    }
}

final fn finalize_foo(a: scalar) {
    let b: group = ChaCha::rand_group();
    values.set(a, b);
}
