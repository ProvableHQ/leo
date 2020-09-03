// Adding the `mut` keyword makes a circuit variable mutable.
circuit Foo {
    mut x: u32
}

function main() {
    let mut a = Foo { x: 1 };
    a.x = 0;

    console.assert(a.x == 0u32);
}