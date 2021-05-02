circuit Foo {
    a: u8,

    function bar() {}
}

function main() {
    let f  = Foo { a: 0u8 };

    f.bar = 1u8;
}