program finalize.aleo {
    struct Foo {
        value: u32,
    }

    struct value {
        owner: u32,
    }

    record value {
        value: u32,
        owner: u32,
    }

    transition value() {}

    async transition output() -> Future {
        return finalize();
    }
    async function finalize() {}

    function key() {}

    inline value() {}

    transition main(f: Foo) -> Foo {
        return f;
    }
}
