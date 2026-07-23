program test.aleo {
    @noupgrade
    constructor() {}
}

export struct Foo {
    x: u8,
}

export fn Foo() {}

export fn main(y: bool) -> bool {
    return y;
}
