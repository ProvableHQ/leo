struct Foo {
    a: u8;
}

function main() {
    const f  = Foo { a: 0u8 };

    f.a = 1u8;
}