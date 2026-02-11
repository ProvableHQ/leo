program const_generics.aleo {
    fn foo() -> (field, Final) {
        let f = final { bar(); };
        return (abc::[0](), f);
    }
}

final fn bar() { }

fn abc::[C: field]() -> field {
    return def::[C]();
}

fn def::[C: field]() -> field {
    return C;
}
