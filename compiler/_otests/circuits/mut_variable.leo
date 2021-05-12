circuit Foo {
    a: u8,
}

function main() {
    let f = Foo { a: 0u8 };

    console.assert(f.a == 0u8);

    f.a = 1u8;

    console.assert(f.a == 1u8);

    f.a = 2u8;

    console.assert(f.a == 2u8);
}