
program test.aleo {

    mapping values: scalar => group;

    async fn foo() -> Future {
        let a: scalar = ChaCha::rand_scalar();
        return finalize_foo(a);
    }
}

async fn finalize_foo(a: scalar) {
    let b: group = ChaCha::rand_group();
    values.set(a, b);
}
