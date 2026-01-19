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

    fn value() {}

    async fn output() -> Future {
        return finalize();
    }

    fn main(f: Foo) -> Foo {
        return f;
    }
}

async fn finalize() {}
fn key() {}
@inline
fn value() {}
