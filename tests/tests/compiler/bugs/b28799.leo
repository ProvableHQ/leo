program bug.aleo {
    @noupgrade
    constructor() {}

    fn bar() -> Foo::[16] {
        return foo::[8]();
    }
}

export struct Foo::[N: u32] {
    x: u32,
}

export fn foo::[N: u32]() -> Foo::[2 * N] {
    return Foo::[2 * N] { x: 0 };
}
