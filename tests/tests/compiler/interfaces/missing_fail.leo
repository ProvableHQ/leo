interface Bar{
    record Foo;
}

interface Baz : Bar {}

interface AbstractFoo : Baz {
    record Foo;
    fn foobar() -> Foo;
}


program scratch.aleo : AbstractFoo {}
