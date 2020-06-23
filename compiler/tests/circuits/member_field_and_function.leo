circuit Foo {
    foo: u32

    static function bar() -> u32 {
        return 0
    }
}

function main() -> u32 {
    let f = Foo { foo: 1 };
    return f.foo + Foo::bar()
}
