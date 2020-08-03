circuit Foo {
    x: u32
}

function main() {
    let a = Foo { x: 1u32 };

    let err = a.y;
}