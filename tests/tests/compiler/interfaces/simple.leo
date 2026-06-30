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
    record Foo {
        owner: address,
        num: u32,
    }

    fn foobar() -> Foo {
        return Foo { owner: self.signer, num: 42 };
    }

    @noupgrade
    constructor() {}
}
