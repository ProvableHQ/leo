program test.aleo {
    // The maximum number of outputs for a `transition` is 16.
    transition bar() -> (
        u8, u8, u8, u8, u8, u8, u8, u8,
        u8, u8, u8, u8, u8, u8, u8, u8,
        u8
    ) {
        return (0, 1, 2, 3, 4, 5, 6, 7,
                8, 9, 10, 11, 12, 13, 14, 15,
                16);
    }

    // The maximum number of outputs for an `async transition` is 16.
    async transition baz() ->
        (u8, u8, u8, u8, u8, u8, u8,
        u8, u8, u8, u8, u8, u8, u8, u8,
        u8, Future)
    {
        return (0, 1, 2, 3, 4, 5, 6, 7,
                8, 9, 10, 11, 12, 13, 14, 15,
                baz_finalize());
    }

    async function baz_finalize() {}

    // The maximum number of outputs for a `function` is 16.
    function foo(x: u8) -> (
        u8, u8, u8, u8, u8, u8, u8, u8,
        u8, u8, u8, u8, u8, u8, u8, u8,
        u8
    ) {
        return (0, 1, 2, 3, 4, 5, 6, 7,
                8, 9, 10, 11, 12, 13, 14, 15,
                16);
    }
}



