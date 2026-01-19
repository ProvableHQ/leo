program const_generics.aleo {
    async fn foo() -> (field, Future) {
        let f = bar();
        return (abc::[0](), f);
    }
}

async fn bar() { }

@inline
fn abc::[C: field]() -> field {
    return def::[C]();
}

@inline
fn def::[C: field]() -> field {
    return C;
}
