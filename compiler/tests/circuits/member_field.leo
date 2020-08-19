circuit Foo {
    x: u32,
}

function main() {
    let a = Foo { x: 1u32 };

    console.assert(a.x == 1u32);
}