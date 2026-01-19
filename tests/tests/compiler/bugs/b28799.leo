program bug.aleo {
    @noupgrade
    async constructor() {}

    fn bar() -> Foo::[16] {
        return foo::[8]();
    }
}

struct Foo::[N: u32] {
    x: u32,
}

@inline
fn foo::[N: u32]() -> Foo::[2 * N] {
    return Foo::[2 * N] { x: 0 };
}

