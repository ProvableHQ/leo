export struct Bar {
    baz: Baz,
}

export struct Baz {
    bar: Bar,
}

program test.aleo {
    @noupgrade
    constructor() {}
}
