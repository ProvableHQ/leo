// Adding the `mut` keyword makes a struct variable mutable.
struct Foo {
    function bar() {}
}

function main() {
    let a = Foo { x: 1 };
    a.bar = 0;
}