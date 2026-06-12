program test.aleo {
    record Token {
        owner: address,
        bar: Bar,
    }

    record Foo {
        owner: address,
        amount: u64,
    }

    @noupgrade
    constructor() {}
}

struct Bar {
    bar: Foo,
}
