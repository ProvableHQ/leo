// Adding the `mut` keyword makes a struct variable mutable.
struct Foo {
    x: u32;
}

function main() {
    let a = Foo { x: 1 };
    a.x = 0;

    console.assert(a.x == 0u32);
}