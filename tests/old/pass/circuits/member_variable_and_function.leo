circuit Foo {
    foo: u32,

    function bar() -> u32 {
        return 1u32
    }
}

function main() {
    const a = Foo { foo: 1 };

    const b = a.foo + Foo::bar();

    console.assert(b == 2u32);
}
