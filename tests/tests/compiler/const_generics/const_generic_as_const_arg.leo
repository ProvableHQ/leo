program const_generics.aleo {
    fn main() -> field {
        return abc::[0]() + abc::[1]();
    }

    @noupgrade
    constructor() {}
}

export fn abc::[C: field]() -> field {
    return def::[C]();
}

export fn def::[C: field]() -> field {
    return C;
}
