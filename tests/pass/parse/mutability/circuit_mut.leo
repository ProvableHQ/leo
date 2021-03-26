// Adding the `mut` keyword makes a circuit variable mutable.
circuit Foo {
    x: u32
}

function main() {
    let a = Foo { x: 1 };
    a.x = 0;

    console.assert(a.x == 0u32);
}