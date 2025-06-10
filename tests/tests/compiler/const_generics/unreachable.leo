program const_generics.aleo {
    transition main() -> field {
        return 0;
    }

    async transition main2() -> Future {
        return foo();
    }

    async function foo() {}

    function bar(x: field) -> field { return x; }

    // Ensure we don't error out on functions that are not accessible from program entry points
    inline abc::[C: field]() -> field {
        return def::[C]();
    }

    inline def::[C: field]() -> field {
        return C;
    }
}
