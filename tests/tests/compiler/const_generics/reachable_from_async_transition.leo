program const_generics.aleo {
    fn foo() -> (field, Final) {
        let f = final { bar(); };
        return (abc::[0](), f);
    }

    @noupgrade
    constructor() {}
}

export final fn bar() {}

export fn abc::[C: field]() -> field {
    return def::[C]();
}

export fn def::[C: field]() -> field {
    return C;
}
