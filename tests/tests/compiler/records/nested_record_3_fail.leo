
program test.aleo {
    record Token {
        owner: address,
        bar: Bar,
    }

    record Foo {
        owner: address,
        amount: u64,
    }
}

struct Bar {
    bar: Foo,
}
