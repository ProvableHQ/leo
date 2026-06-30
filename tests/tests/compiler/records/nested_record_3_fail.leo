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

export struct Bar {
    bar: Foo,
}
