struct Foo {
    value: u32,
}

struct value {
    owner: u32,
}

program finalize.aleo {
    record value {
        value: u32,
        owner: u32,
    }

    transition value() {}

    async transition output() -> Future {
        return finalize();
    }

    transition main(f: Foo) -> Foo {
        return f;
    }
}

async function finalize() {}
function key() {}
inline value() {}
