program const_generics.aleo {
    transition foo() {}

    function bar(x: field) -> field { return abc::[0](); }

    inline abc::[C: field]() -> field {
        return def::[C]();
    }

    inline def::[C: field]() -> field {
        return C;
    }
}
