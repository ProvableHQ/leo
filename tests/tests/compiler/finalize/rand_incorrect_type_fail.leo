
program test.aleo {

    mapping values: scalar => group;

    async fn foo() -> Future {
        return finalize_foo();
    }
}

async fn finalize_foo() {
    let a: scalar = ChaCha::rand_field();
    let b: group = ChaCha::rand_field();
    values.set(a, b);
}
