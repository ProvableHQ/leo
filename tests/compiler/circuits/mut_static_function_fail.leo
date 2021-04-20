circuit Foo {
    function bar() {}
}

function main() {
    let f = Foo { a: 0u8 };

    f.bar = 1u8;
}