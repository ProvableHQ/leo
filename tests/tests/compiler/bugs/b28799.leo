program bug.aleo {
    @noupgrade
    constructor() {}

    fn bar() -> Foo::[16] {
        return foo::[8]();
    }
}

struct Foo::[N: u32] {
    x: u32,
}

fn foo::[N: u32]() -> Foo::[2 * N] {
    return Foo::[2 * N] { x: 0 };
}

