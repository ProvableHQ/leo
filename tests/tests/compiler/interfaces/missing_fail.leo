export interface Bar {
    record Foo;
}

export interface Baz: Bar {
}

export interface AbstractFoo: Baz {
    record Foo;
    fn foobar() -> Foo;
}

program scratch.aleo: AbstractFoo {
    @noupgrade
    constructor() {}
}
