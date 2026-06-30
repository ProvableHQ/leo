program const_generics.aleo {
    fn main() -> field {
        return 0;
    }

    fn main2() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}

export final fn foo() {}

export fn bar(x: field) -> field {
    return x;
}

// Ensure we don't error out on functions that are not accessible from program entry points
export fn abc::[C: field]() -> field {
    return def::[C]();
}

export fn def::[C: field]() -> field {
    return C;
}
