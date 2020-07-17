circuit Foo {
    f: u32,

    function bar() -> u32 {
        return self.f
    }
}

function main() -> u32 {
    let foo = Foo { f: 1u32 };
    return foo.bar()
}