// Adding the `mut` keyword makes a circuit variable mutable.
circuit Foo {
    function bar() {}
}

function main() {
    let a = Foo { x: 1 };
    a.bar = 0;
}