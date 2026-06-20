program test.aleo {
    @noupgrade
    constructor() {}
}

export struct Foo {
    foo: [Foo; 1],
}

export struct Bar {
    baz: [Baz, 2],
}

export struct Baz {
    bar: [Bar, 3],
}

export struct One {
    two: [Two, 2],
}

export struct Two {
    three: [Three, 3],
    four: [Four, 4],
}

export struct Three {
    one: [One, 1],
}

export struct Four {
    one: [One, 1],
}
