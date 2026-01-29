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

    fn output() -> Final {
        return finalize();
    }

    fn main(f: Foo) -> Foo {
        return f;
    }
}

onchain fn finalize() {}
@no_inline
fn key() {}
fn value() {}
