program const_generics.aleo {
    fn foo() {}

    @noupgrade
    constructor() {}
}

fn bar(x: field) -> field {
    return abc::[0]();
}

fn abc::[C: field]() -> field {
    return def::[C]();
}

fn def::[C: field]() -> field {
    return C;
}
