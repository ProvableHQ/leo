circuit Foo {
    f: u32,

    function bar() -> u32 {
        return f
    }
}

function main() -> u32 {
    let foo = Foo { f: 1u32 };
    let err = foo.bar();
}