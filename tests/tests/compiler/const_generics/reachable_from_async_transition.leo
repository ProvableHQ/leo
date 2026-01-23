program const_generics.aleo {
    fn foo() -> (field, Future) {
        let f = bar();
        return (abc::[0](), f);
    }
}

onchain fn bar() { }

fn abc::[C: field]() -> field {
    return def::[C]();
}

fn def::[C: field]() -> field {
    return C;
}
