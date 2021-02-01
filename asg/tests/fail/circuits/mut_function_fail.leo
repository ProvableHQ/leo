circuit Foo {
    a: u8,

    function bar() {}
}

function main() {
    let mut f  = Foo { a: 0u8 };

    f.bar = 1u8;
}