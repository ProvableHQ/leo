
program test.aleo {

    mapping values: scalar => group;

    fn foo() -> Future {
        return finalize_foo();
    }
}

onchain fn finalize_foo() {
    let a: scalar = ChaCha::rand_field();
    let b: group = ChaCha::rand_field();
    values.set(a, b);
}
