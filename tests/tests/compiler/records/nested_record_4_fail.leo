program test.aleo {
    record Token3 {
        owner: address,
        bar: (Bar, Bar),
    }

    record Token2 {
        owner: address,
        amount: u64,
    }

    @noupgrade
    constructor() {}
}

export struct Bar {
    bar: (Token2, Token2),
}
