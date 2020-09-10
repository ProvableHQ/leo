circuit Foo {
    a: u8,
}

function main() {
    let mut f  = Foo { a: 0u8 };

    f.a = 1u8;
}