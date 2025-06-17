program const_generics.aleo {
    async transition foo() -> (field, Future) {
        let f = bar();
        return (abc::[0](), f);
    }

    async function bar() { }

    inline abc::[C: field]() -> field {
        return def::[C]();
    }

    inline def::[C: field]() -> field {
        return C;
    }
}
