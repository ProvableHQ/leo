// Circuits are immutable by default.
struct Foo {
    x: u32;
}

function main() {
    const a = Foo { x: 1 };
    a.x = 0;
}