circuit Foo {
    function bar() -> u32 {
        return self.f
    }
}

function main() -> u32 {
    let foo = Foo { };
    return foo.bar()
}