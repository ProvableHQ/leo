program const_generics.aleo {
    fn foo() {}

    @noupgrade
    constructor() {}
}

export fn bar(x: field) -> field {
    return abc::[0]();
}

export fn abc::[C: field]() -> field {
    return def::[C]();
}

export fn def::[C: field]() -> field {
    return C;
}
